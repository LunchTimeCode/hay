use std::fs;

use crate::config;

pub fn example() -> anyhow::Result<String> {
    let c = config::Config::example();
    let as_toml = toml::to_string_pretty(&c)?;
    fs::write("hy.toml", as_toml).expect("Unable to write file");
    Ok("file created".to_string())
}
