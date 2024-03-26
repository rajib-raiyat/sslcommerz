use std::fs::File;
use std::io::Read;

use lazy_static::lazy_static;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub payment_credentials: PaymentCredentials,
}

#[derive(Debug, Deserialize)]
pub struct PaymentCredentials {
    pub base_url: String,
    pub store_id: String,
    pub store_passwd: String,
    pub success_url: String,
    pub fail_url: String,
    pub cancel_url: String,
    pub ipn_url: Option<String>,
}

pub fn load_config() -> Result<Config, toml::de::Error> {
    let mut file = File::open("config.toml").expect("Unable to open `config.toml` file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read `config.toml` file");

    toml::from_str(&contents)
}

lazy_static! {
    pub static ref CONFIG: Config = load_config().expect("Failed to load `config.toml` config.");
}
