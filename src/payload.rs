use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Model {
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) display_name: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct ContextWindow {
    #[serde(default)]
    pub(crate) total_input_tokens: usize,
    #[serde(default)]
    pub(crate) total_output_tokens: usize,
    #[serde(default)]
    pub(crate) used_percentage: f64,
}

#[derive(Deserialize, Default, Debug)]
pub struct QuotaInfo {
    #[serde(default)]
    pub(crate) remaining_fraction: f64,
    #[serde(default)]
    pub(crate) reset_in_seconds: i64,
}

#[derive(Deserialize, Default, Debug)]
pub struct Payload {
    #[serde(default)]
    pub(crate) cwd: String,
    #[serde(default)]
    pub(crate) terminal_width: usize,
    #[serde(default)]
    pub(crate) model: Model,
    #[serde(default)]
    pub(crate) context_window: ContextWindow,
    #[serde(default)]
    pub(crate) quota: HashMap<String, QuotaInfo>,
}
