use std::{
    io,
    process::{Command, Stdio},
};

use crate::display::DisplaySettings;

pub(crate) fn build_branch_block(cwd: &str, display: &DisplaySettings) -> Option<String> {
    let output = match Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(cwd)
        .stderr(Stdio::piped())
        .output()
    {
        Ok(output) => output,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Some(display.wrap_color("[Git Command Not Found]", "31"));
        }
        Err(_) => return None,
    };

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr).to_lowercase();
        if err_msg.contains("not a git repository") || err_msg.contains("not a git repo") {
            return None;
        }

        return Some(display.wrap_color("[Git Error]", "31"));
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() {
        return None;
    }

    let branch = truncate_middle(&branch, 20);
    let dirty_star = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(cwd)
        .output()
        .ok()
        .filter(|status_out| {
            !status_out.stdout.is_empty()
                && !String::from_utf8_lossy(&status_out.stdout)
                    .trim()
                    .is_empty()
        })
        .map(|_| "*")
        .unwrap_or("");

    let branch_text = format!("{}{}", display.module_prefix("branch"), branch);
    if dirty_star.is_empty() {
        Some(display.wrap_color(&format!("[{}]", branch_text), "34"))
    } else if display.color {
        Some(format!(
            "\x1b[34m[{}\x1b[31m{}\x1b[34m]\x1b[0m",
            branch_text, dirty_star
        ))
    } else {
        Some(format!("[{}{}]", branch_text, dirty_star))
    }
}

fn truncate_middle(value: &str, max_chars: usize) -> String {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() <= max_chars {
        return value.to_string();
    }

    let keep = (max_chars - 3) / 2;
    let start: String = chars[..keep + 1].iter().collect();
    let end: String = chars[chars.len() - keep..].iter().collect();
    format!("{}...{}", start, end)
}
