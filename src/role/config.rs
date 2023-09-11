use super::resp::{Data, Resp};
use super::user::*;
use anyhow::anyhow;
use anyhow::{Context, Result};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

/// Info struct is used to store the information of the user's state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    username: String,
    password: String,
    cookie: String,
    user: Option<User>,
}

impl Config {
    pub fn new(username: String, password: String, cookie: String, user: Option<User>) -> Self {
        Config {
            username,
            password,
            cookie,
            user,
        }
    }

    pub fn username(&self) -> &str {
        self.username.as_ref()
    }

    pub fn password(&self) -> &str {
        self.password.as_ref()
    }

    pub fn cookie(&self) -> &str {
        self.cookie.as_ref()
    }
}

pub fn save_config_to_file(config: &Config) -> Result<Resp> {
    let root = home_dir().context("can not get home dir")?;
    let path = root.join(".njfu-library-cli.json");

    let mut output = File::create(path)?;
    let info = serde_json::to_string_pretty(&config)?;
    write!(output, "{}", info)?;
    Ok(Resp::new(
        0,
        "save cookie success".to_string(),
        Some(vec![Data::Config(config.clone())]),
    ))
}

pub fn load_config_from_file() -> Result<Resp> {
    let root = home_dir().context("can not get home dir")?;
    let path = root.join(".njfu-library-cli.json");

    let input = File::open(path).context("please login first")?;
    let mut config: Config = serde_json::from_reader(input)?;

    if config.username.is_empty() || config.password.is_empty() || config.cookie.is_empty() {
        return Err(anyhow!("please login first"));
    }

    if config.user.is_none() {
        let user = search_user_info(&config)?;
        config.user = Some(user);
        save_config_to_file(&config)?;
    }

    Ok(Resp::new(
        0,
        "load cookie success".to_string(),
        Some(vec![Data::Config(config)]),
    ))
}