use std::{env, path::Path};

use crossterm::terminal;

use crate::{
    config::EffectiveModule,
    display::{DisplaySettings, IconStyle, LayoutMode},
    git::build_branch_block,
    payload::Payload,
};

pub fn render_payload(
    payload: &Payload,
    modules: &[EffectiveModule],
    display: &DisplaySettings,
) -> String {
    let cwd = resolve_cwd(payload);
    let dir_path = Path::new(&cwd);
    let dir_name = dir_path.file_name().unwrap_or_default().to_string_lossy();
    let model_name = model_name(payload);
    let token_display = token_display(payload);
    let quota_block = quota_block(payload, &model_name, display);

    let mut blocks = Vec::new();
    for module in modules {
        match module.id {
            "dir" => blocks.push(display.wrap_color(
                &format!("{}{}", display.module_prefix("dir"), dir_name),
                "36",
            )),
            "branch" => {
                if let Some(branch_block) = build_branch_block(&cwd, display) {
                    blocks.push(branch_block);
                }
            }
            "model" => blocks.push(display.wrap_color(
                &format!("{}{}", display.module_prefix("model"), model_name),
                "35",
            )),
            "tokens" => blocks.push(display.wrap_color(
                &format!("{}{}", display.module_prefix("tokens"), token_display),
                "33",
            )),
            "quota" => blocks.push(quota_block.clone()),
            _ => {}
        }
    }

    format_statusline(&blocks, terminal_width(payload), display.layout, display)
}

fn resolve_cwd(payload: &Payload) -> String {
    let mut cwd = payload.cwd.clone();
    if cwd.is_empty()
        && let Ok(current_dir) = env::current_dir()
    {
        cwd = current_dir.to_string_lossy().to_string();
    }
    cwd
}

fn model_name(payload: &Payload) -> String {
    let mut model_name = payload.model.display_name.clone();
    if model_name.is_empty() {
        model_name = payload.model.id.clone();
    }

    if model_name.is_empty() {
        "Unknown Model".to_string()
    } else {
        model_name
    }
}

fn token_display(payload: &Payload) -> String {
    let total_token =
        payload.context_window.total_input_tokens + payload.context_window.total_output_tokens;
    format!(
        "{} ({:.1}%)",
        format_tokens(total_token),
        payload.context_window.used_percentage
    )
}

fn quota_block(payload: &Payload, model_name: &str, display: &DisplaySettings) -> String {
    let mut quota_5h = -1.0;
    let mut quota_7d = -1.0;
    let mut reset_sec_5h = 0;
    let mut reset_sec_7d = 0;

    let (key_5h, key_7d) = if model_name.to_lowercase().contains("gemini") {
        ("gemini-5h", "gemini-weekly")
    } else {
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

    let q5h_str = make_quota_badge(quota_5h, "5h", reset_sec_5h, display);
    let q7d_str = make_quota_badge(quota_7d, "7d", reset_sec_7d, display);
    let quota_values = [q5h_str, q7d_str]
        .into_iter()
        .filter(|quota| !quota.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    if quota_values.is_empty() {
        format!(
            "{}N/A",
            display.wrap_color(&display.module_prefix("quota"), "32")
        )
    } else {
        format!(
            "{}{}",
            display.wrap_color(&display.module_prefix("quota"), "32"),
            quota_values
        )
    }
}

fn terminal_width(payload: &Payload) -> usize {
    if payload.terminal_width != 0 {
        payload.terminal_width
    } else if let Ok((column, _)) = terminal::size() {
        column as usize
    } else {
        80
    }
}

fn format_tokens(num: usize) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}

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

fn make_quota_badge(
    fraction: f64,
    label: &str,
    reset_sec: i64,
    display: &DisplaySettings,
) -> String {
    if fraction < 0.0 {
        return String::new();
    }
    let pct = fraction * 100.0;
    let color_code = if pct <= 20.0 {
        "31"
    } else if pct <= 50.0 {
        "33"
    } else {
        "32"
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

    display.wrap_color(
        &format!("{} {} {:.1}%{}", label, bar_str, pct, reset_str),
        color_code,
    )
}

fn format_statusline(
    blocks: &[String],
    cols: usize,
    layout: LayoutMode,
    display: &DisplaySettings,
) -> String {
    let split_lines = |blocks: &[String]| {
        let mid = blocks.len().div_ceil(2);
        let line1 = blocks[..mid].join(" | ");
        let line2 = blocks[mid..].join(" | ");
        let (line1_marker, line2_marker) = if display.icons == IconStyle::Emoji {
            ("╭─", "╰─")
        } else {
            ("+-", "+-")
        };

        format!(
            "{} {}\n{} {}",
            display.wrap_color(line1_marker, "90"),
            line1,
            display.wrap_color(line2_marker, "90"),
            line2
        )
    };

    match layout {
        LayoutMode::SingleLine => blocks.join(" | "),
        LayoutMode::TwoLines if blocks.len() > 1 => split_lines(blocks),
        LayoutMode::TwoLines => blocks.join(" | "),
        LayoutMode::Auto if cols < 100 && blocks.len() > 1 => split_lines(blocks),
        LayoutMode::Auto => blocks.join(" | "),
    }
}
