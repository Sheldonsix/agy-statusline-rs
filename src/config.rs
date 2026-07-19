use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::display::DisplayConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Macos,
    Windows,
    Unix,
}

#[derive(Debug, Clone, Copy)]
struct ModuleDefinition {
    id: &'static str,
    default_order: u32,
}

const MODULE_DEFINITIONS: [ModuleDefinition; 5] = [
    ModuleDefinition {
        id: "dir",
        default_order: 10,
    },
    ModuleDefinition {
        id: "branch",
        default_order: 20,
    },
    ModuleDefinition {
        id: "model",
        default_order: 30,
    },
    ModuleDefinition {
        id: "tokens",
        default_order: 40,
    },
    ModuleDefinition {
        id: "quota",
        default_order: 50,
    },
];

#[derive(Deserialize, Default, Debug)]
pub struct UserConfig {
    #[serde(default)]
    pub modules: HashMap<String, ModuleConfig>,
    #[serde(default)]
    pub display: DisplayConfig,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct ModuleConfig {
    pub enabled: Option<bool>,
    pub order: Option<u32>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CliOverrides {
    pub hide_quota: bool,
    pub hide_model: bool,
    pub hide_tokens: bool,
    pub hide_dir: bool,
    pub hide_branch: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectiveModule {
    pub id: &'static str,
    pub order: u32,
}

pub fn current_platform() -> Platform {
    if cfg!(target_os = "macos") {
        Platform::Macos
    } else if cfg!(target_os = "windows") {
        Platform::Windows
    } else {
        // Linux and WSL both follow the XDG-style CLI convention.
        Platform::Unix
    }
}

pub fn default_config_path_for<F>(platform: Platform, get_env: &F) -> Option<PathBuf>
where
    F: Fn(&str) -> Option<OsString>,
{
    match platform {
        Platform::Unix => {
            let base = non_empty_path(get_env("XDG_CONFIG_HOME"))
                .or_else(|| non_empty_path(get_env("HOME")).map(|home| home.join(".config")))?;
            Some(base.join("agy-statusline").join("config.json"))
        }
        Platform::Macos => {
            let home = non_empty_path(get_env("HOME"))?;
            Some(
                home.join("Library")
                    .join("Application Support")
                    .join("agy-statusline")
                    .join("config.json"),
            )
        }
        Platform::Windows => {
            let appdata = non_empty_path(get_env("APPDATA"))?;
            Some(appdata.join("agy-statusline").join("config.json"))
        }
    }
}

pub fn resolve_config_path_with<F>(get_env: &F) -> Option<PathBuf>
where
    F: Fn(&str) -> Option<OsString>,
{
    non_empty_path(get_env("AGY_STATUSLINE_CONFIG"))
        .or_else(|| default_config_path_for(current_platform(), get_env))
}

pub fn resolve_config_path() -> Option<PathBuf> {
    resolve_config_path_with(&|key| env::var_os(key))
}

pub fn parse_user_config(raw: &str) -> Result<Option<UserConfig>, serde_json::Error> {
    if raw.trim().is_empty() {
        return Ok(None);
    }

    serde_json::from_str(raw).map(Some)
}

pub fn load_user_config(explicit_path: Option<&Path>) -> UserConfig {
    let config_path = explicit_path
        .map(PathBuf::from)
        .or_else(resolve_config_path);
    let Some(path) = config_path else {
        return UserConfig::default();
    };

    let raw = match fs::read_to_string(&path) {
        Ok(raw) => raw,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return UserConfig::default(),
        Err(error) => {
            eprintln!(
                "Statusline config warning: failed to read {}: {}",
                path.display(),
                error
            );
            return UserConfig::default();
        }
    };

    match parse_user_config(&raw) {
        Ok(Some(config)) => config,
        Ok(None) => UserConfig::default(),
        Err(error) => {
            eprintln!(
                "Statusline config warning: failed to parse {}: {}",
                path.display(),
                error
            );
            UserConfig::default()
        }
    }
}

pub fn effective_modules(config: &UserConfig, overrides: CliOverrides) -> Vec<EffectiveModule> {
    let mut modules = MODULE_DEFINITIONS
        .iter()
        .filter_map(|definition| {
            let user_module = config.modules.get(definition.id);
            let enabled = user_module
                .and_then(|module| module.enabled)
                .unwrap_or(true);

            if !enabled || cli_hides_module(overrides, definition.id) {
                return None;
            }

            Some(EffectiveModule {
                id: definition.id,
                order: user_module
                    .and_then(|module| module.order)
                    .unwrap_or(definition.default_order),
            })
        })
        .collect::<Vec<_>>();

    modules.sort_by_key(|module| (module.order, module.id));
    modules
}

fn non_empty_path(value: Option<OsString>) -> Option<PathBuf> {
    value.and_then(|value| {
        if value.as_os_str().is_empty() {
            None
        } else {
            Some(PathBuf::from(value))
        }
    })
}

fn cli_hides_module(overrides: CliOverrides, id: &str) -> bool {
    match id {
        "dir" => overrides.hide_dir,
        "branch" => overrides.hide_branch,
        "model" => overrides.hide_model,
        "tokens" => overrides.hide_tokens,
        "quota" => overrides.hide_quota,
        _ => false,
    }
}
