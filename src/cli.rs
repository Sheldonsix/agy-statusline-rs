use std::path::PathBuf;

use clap::Parser;

use crate::config::CliOverrides;

#[derive(Parser, Debug)]
#[command(author, version, about = "A custom statusline for Antigravity")]
pub struct Args {
    /// 配置文件路径；未指定时读取 AGY_STATUSLINE_CONFIG 或平台默认位置
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// 是否隐藏 Quota (默认显示)
    #[arg(long, default_value_t = false)]
    pub hide_quota: bool,

    /// 是否隐藏模型信息 (默认显示)
    #[arg(long, default_value_t = false)]
    pub hide_model: bool,

    /// 是否隐藏 Token 信息 (默认显示)
    #[arg(long, default_value_t = false)]
    pub hide_tokens: bool,

    /// 是否隐藏目录名 (默认显示)
    #[arg(long, default_value_t = false)]
    pub hide_dir: bool,

    /// 是否隐藏 Git 分支名 (默认显示)
    #[arg(long, default_value_t = false)]
    pub hide_branch: bool,
}

impl Args {
    pub fn cli_overrides(&self) -> CliOverrides {
        CliOverrides {
            hide_quota: self.hide_quota,
            hide_model: self.hide_model,
            hide_tokens: self.hide_tokens,
            hide_dir: self.hide_dir,
            hide_branch: self.hide_branch,
        }
    }
}
