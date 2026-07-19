use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    path::Path,
    process::{Command, exit},
};

use crossterm::terminal;
use serde::Deserialize;

/// 定义需要解析的 json 格式
#[derive(Deserialize, Default, Debug)]
struct Model {
    #[serde(default)]
    id: String,
    #[serde(default)]
    display_name: String,
}

#[derive(Deserialize, Default, Debug)]
struct ContextWindow {
    #[serde(default)]
    total_input_tokens: usize,
    #[serde(default)]
    total_output_tokens: usize,
    #[serde(default)]
    used_percentage: f64,
}

#[derive(Deserialize, Default, Debug)]
struct QuotaInfo {
    #[serde(default)]
    remaining_fraction: f64,
    #[serde(default)]
    reset_in_seconds: i64,
}

#[derive(Deserialize, Default, Debug)]
struct Payload {
    #[serde(default)]
    cwd: String,
    #[serde(default)]
    terminal_width: usize,
    #[serde(default)]
    model: Model,
    #[serde(default)]
    context_window: ContextWindow,
    #[serde(default)]
    quota: HashMap<String, QuotaInfo>,
    #[serde(default)]
    version: String,
    #[serde(default)]
    email: String,
}

/// 格式化 Token 数量
fn format_tokens(num: usize) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}

// 格式化倒计时
fn format_reset_time(sec: i64) -> String {
    if sec <= 0 {
        return String::new();
    }

    let days = sec / 86400;
    let rem = sec % 86400;
    let hours = rem / 3600;
    let rem = rem % 3600;
    let mins = rem / 60;

    if days > 0 {
        if hours > 0 {
            format!("{}d {}h", days, hours)
        } else {
            format!("{}d", days)
        }
    } else if hours > 0 {
        if mins > 0 {
            format!("{}h {}m", hours, mins)
        } else {
            format!("{}h", hours)
        }
    } else {
        format!("{}m", mins.max(1))
    }
}

/// quota 进度条生成
fn make_quota_badge(fraction: f64, label: &str, reset_sec: i64) -> String {
    if fraction < 0.0 {
        return String::new();
    }
    let pct = fraction * 100.0;
    let color = if pct <= 20.0 {
        "\x1b[31m" // 红色
    } else if pct <= 50.0 {
        "\x1b[33m" // 黄色
    } else {
        "\x1b[32m" // 绿色
    };

    let total_bars = 5;
    let full_bars = (fraction * (total_bars as f64)) as usize;

    let mut bar_str = String::new();
    for i in 0..total_bars {
        if i < full_bars + 1 {
            bar_str.push('■');
        } else {
            bar_str.push('□');
        }
    }

    let reset_str = if reset_sec > 0 {
        format!("({})", format_reset_time(reset_sec))
    } else {
        String::new()
    };

    format!(
        "{}{} {} {:.1}%\x1b[0m{}",
        color, label, bar_str, pct, reset_str
    )
}

fn main() {
    // 接收标准输入
    let mut input_data = Vec::new();
    io::stdin().read_to_end(&mut input_data).unwrap_or(0);

    let input_str = String::from_utf8_lossy(&input_data);
    if input_str.trim().is_empty() {
        exit(0);
    }

    // 解析 JSON
    let payload: Payload = match serde_json::from_str(&input_str) {
        Ok(p) => p,

        Err(e) => {
            println!("\x1b[31mStatusline Error: {}\x1b[0m", e);
            exit(1);
        }
    };

    // 获取目录名称和 Git 分支
    let mut cwd = payload.cwd.clone();
    if cwd.is_empty() {
        if let Ok(current_dir) = env::current_dir() {
            cwd = current_dir.to_string_lossy().to_string();
        }
    }

    let dir_path = Path::new(&cwd);
    let dir_name = dir_path.file_name().unwrap_or_default().to_string_lossy();

    let mut branch_display = String::new();
    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(&cwd)
        .output()
    {
        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            branch_display = format!(" \x1b[31m[Git Error:{}]\x1b[0m", err_msg.trim());
        } else {
            let mut branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !branch.is_empty() {
                // 截断过长的分支名称
                let chars: Vec<char> = branch.chars().collect();
                if chars.len() > 20 {
                    let keep = (20 - 3) / 2;
                    let p1: String = chars[..keep + 1].iter().collect();
                    let p2: String = chars[chars.len() - keep..].iter().collect();
                    branch = format!("{}...{}", p1, p2);
                }

                // 检查是否有更改
                let mut dirty_star = "";
                if let Ok(status_out) = Command::new("git")
                    .args(["status", "--porcelain"])
                    .current_dir(&cwd)
                    .output()
                {
                    if !status_out.stdout.is_empty()
                        && !String::from_utf8_lossy(&status_out.stdout)
                            .trim()
                            .is_empty()
                    {
                        dirty_star = "*";
                    }
                }

                branch_display = format!(
                    " \x1b[34m[🌿 {}\x1b[31m{}\x1b[34m]\x1b[0m",
                    branch, dirty_star
                );
            }
        }
    } else {
        branch_display = " \x1b[31m[Git Command Not Found]\x1b[0m".to_string();
    }

    // 获取当前模型
    let mut model_name = payload.model.display_name.clone();
    if model_name.is_empty() {
        model_name = payload.model.id.clone();
    }

    if model_name.is_empty() {
        model_name = "Unknown Model".to_string();
    }

    // 获取 Token 数量及占比
    let total_token =
        payload.context_window.total_input_tokens + payload.context_window.total_output_tokens;
    let token_display = format!(
        "{} ({:.1}%)",
        format_tokens(total_token),
        payload.context_window.used_percentage
    );

    // 获取 5h 和 weekly（7d）的剩余额度
    let mut quota_5h = -1.0;
    let mut quota_7d = -1.0;
    let mut reset_sec_5h = 0;
    let mut reset_sec_7d = 0;

    let (key_5h, key_7d) = if model_name.to_lowercase().contains("gemini") {
        // Gemini 模型
        ("gemini-5h", "gemini-weekly")
    } else {
        // 其他模型
        ("3p-5h", "3p-weekly")
    };

    if let Some(q) = payload.quota.get(key_5h) {
        quota_5h = q.remaining_fraction;
        reset_sec_5h = q.reset_in_seconds;
    }
    if let Some(q) = payload.quota.get(key_7d) {
        quota_7d = q.remaining_fraction;
        reset_sec_7d = q.reset_in_seconds;
    }

    let q5h_str = make_quota_badge(quota_5h, "5h", reset_sec_5h);
    let q7d_str = make_quota_badge(quota_7d, "7d", reset_sec_7d);

    let mut quota_block = "\x1b[32m⚡ Quota: N/A\x1b[0m".to_string();
    if !q5h_str.is_empty() || !q7d_str.is_empty() {
        quota_block = format!("\x1b[32m⚡ Quota: {} {}", q5h_str, q7d_str)
            .trim()
            .to_string();
    }

    // 拼接输出字符串
    let dir_block = format!("\x1b[36m📁 {}\x1b[0m{}", dir_name, branch_display);
    let model_block = format!("\x1b[35m🤖 {}\x1b[0m", model_name);
    let token_block = format!("\x1b[33m🧠 Tokens: {}\x1b[0m", token_display);

    let cols = if payload.terminal_width != 0 {
        payload.terminal_width
    } else if let Ok((column, _)) = terminal::size() {
        column as usize
    } else {
        80
    };

    let output = if cols < 100 {
        // 屏幕宽度较窄，两行展示
        format!(
            "\x1b[90m╭─\x1b[0m {} | {}\n\x1b[90m╰─\x1b[0m {} | {}",
            dir_block, model_block, token_block, quota_block
        )
    } else {
        // 屏幕宽裕，一行展示
        format!(
            "{} | {} | {} | {}",
            dir_block, model_block, token_block, quota_block
        )
    };

    println!("{}", output);
}
