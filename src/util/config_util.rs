use crate::model::config::Config;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use std::fs;

impl Config {
    pub fn from_str(s: &str) -> Result<Config> {
        toml::from_str(&s).with_context(|| "fail to parse config file!")
    }

    pub fn from_file() -> Result<Config> {
        let config_str = fs::read_to_string("config.toml")
            .with_context(|| format!("Failed to read the config file: config.toml"))?;
        Config::from_str(&config_str).with_context(|| "Configuration is invalid")
    }
}

lazy_static! {
    pub static ref CFG: Config = Config::from_file().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_database() {
        let confg = Config::from_file().unwrap();
        assert_eq!("root", confg.database.user);
    }
}
