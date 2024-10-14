use clap::Parser;
use serde::Deserialize;
use crate::drivers::onedrive::OnedriveConfig;

#[derive(Debug, Deserialize)]
pub struct SearchConfig {
    pub host: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Account {
    Onedrive(OnedriveConfig)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub search: SearchConfig,
    pub account: Vec<Account>,
    #[serde(default = "default_listen")]
    pub listen: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_refresh_time")]
    pub refresh_time: i64,
}

fn default_listen() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_refresh_time() -> i64 {
    3600
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, long, default_value = "config.json")]
    pub config_file: String,
}