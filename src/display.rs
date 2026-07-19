use std::ffi::OsString;

use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone, Copy)]
pub struct DisplayConfig {
    pub color: Option<ColorMode>,
    pub icons: Option<IconMode>,
    pub layout: Option<LayoutMode>,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IconMode {
    Auto,
    Emoji,
    Ascii,
    None,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LayoutMode {
    Auto,
    SingleLine,
    TwoLines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconStyle {
    Emoji,
    Ascii,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplaySettings {
    pub color: bool,
    pub icons: IconStyle,
    pub layout: LayoutMode,
}

impl DisplaySettings {
    pub fn from_config<F>(config: &DisplayConfig, get_env: &F) -> Self
    where
        F: Fn(&str) -> Option<OsString>,
    {
        let term_is_dumb = env_equals(get_env, "TERM", "dumb");
        let color = match config.color.unwrap_or(ColorMode::Auto) {
            ColorMode::Auto => !env_present(get_env, "NO_COLOR") && !term_is_dumb,
            ColorMode::Always => true,
            ColorMode::Never => false,
        };
        let icons = match config.icons.unwrap_or(IconMode::Auto) {
            IconMode::Auto => {
                if term_is_dumb {
                    IconStyle::Ascii
                } else {
                    IconStyle::Emoji
                }
            }
            IconMode::Emoji => IconStyle::Emoji,
            IconMode::Ascii => IconStyle::Ascii,
            IconMode::None => IconStyle::None,
        };
        let layout = config.layout.unwrap_or(LayoutMode::Auto);

        Self {
            color,
            icons,
            layout,
        }
    }

    pub fn icon(&self, module_id: &str) -> &'static str {
        match self.icons {
            IconStyle::Emoji => match module_id {
                "dir" => "📁 ",
                "branch" => "🌿 ",
                "model" => "🤖 ",
                "tokens" => "🧠 ",
                "quota" => "⚡ ",
                _ => "",
            },
            IconStyle::Ascii | IconStyle::None => "",
        }
    }

    pub fn module_prefix(&self, module_id: &str) -> String {
        let label = match (self.icons, module_id) {
            (IconStyle::Emoji, "tokens") => "Tokens: ",
            (IconStyle::Emoji, "quota") => "Quota: ",
            (IconStyle::Emoji, _) => "",
            (_, "dir") => "Dir: ",
            (_, "branch") => "Git: ",
            (_, "model") => "Model: ",
            (_, "tokens") => "Tokens: ",
            (_, "quota") => "Quota: ",
            _ => "",
        };

        format!("{}{}", self.icon(module_id), label)
    }

    pub fn wrap_color(&self, text: &str, code: &str) -> String {
        if self.color {
            format!("\x1b[{}m{}\x1b[0m", code, text)
        } else {
            text.to_string()
        }
    }
}

fn env_equals<F>(get_env: &F, key: &str, expected: &str) -> bool
where
    F: Fn(&str) -> Option<OsString>,
{
    get_env(key)
        .and_then(|value| value.into_string().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case(expected))
}

fn env_present<F>(get_env: &F, key: &str) -> bool
where
    F: Fn(&str) -> Option<OsString>,
{
    get_env(key).is_some()
}
