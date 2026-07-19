pub mod cli;
pub mod config;
pub mod display;
pub mod git;
pub mod payload;
pub mod render;

pub fn render_statusline_from_input(
    input_data: &[u8],
    args: &cli::Args,
) -> Result<Option<String>, String> {
    let config = config::load_user_config(args.config.as_deref());
    let display =
        display::DisplaySettings::from_config(&config.display, &|key| std::env::var_os(key));
    let modules = config::effective_modules(&config, args.cli_overrides());

    let input_str = String::from_utf8_lossy(input_data);
    if input_str.trim().is_empty() {
        return Ok(None);
    }

    let payload: payload::Payload =
        serde_json::from_str(&input_str).map_err(|error| format!("Statusline Error: {error}"))?;

    Ok(Some(render::render_payload(&payload, &modules, &display)))
}
