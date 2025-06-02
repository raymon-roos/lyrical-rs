use crate::token::Error::{FileEmpty, FileUnreadable};
use std::fs::read_to_string;

/// Read the Genius API token from `$XDG_CONFIG_HOME/lyrical/token`.
///
/// If `$XDG_CONFIG_HOME` is not set, the fallback value is `$HOME/.config`.
pub fn read_from_file() -> Result<String, Error> {
    let token_path = config_directory() + "/lyrical/token";

    let token = read_to_string(&token_path).map_err(|_| FileUnreadable(token_path.clone()))?;

    Ok(token
        .lines()
        .next()
        .ok_or(FileEmpty(token_path))?
        .to_string())
}

fn config_directory() -> String {
    match std::env::var("XDG_CONFIG_HOME") {
        Ok(xdg_config_home) => String::from(xdg_config_home),
        Err(_) => format!("{}/.config", std::env::var("HOME").unwrap_or_default()),
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Error {
    FileUnreadable(String),
    FileEmpty(String),
}
