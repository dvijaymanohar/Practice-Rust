use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

// #[derive(Debug, Deserialize)]
// #[allow(unused)]
// struct Database {
//     url: String,
// }

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Sparkpost {
    key: String,
    token: String,
    url: String,
    version: u8,
}

// #[derive(Debug, Deserialize)]
// #[allow(unused)]
// struct Twitter {
//     consumer_token: String,
//     consumer_secret: String,
// }

// #[derive(Debug, Deserialize)]
// #[allow(unused)]
// struct Braintree {
//     merchant_id: String,
//     public_key: String,
//     private_key: String,
// }

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    // debug: bool,
    // database: Database,
    sparkpost: Sparkpost,
    // twitter: Twitter,
    // braintree: Braintree,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        // Now that we're done, let's access our configuration
        println!("Sparkpost key: {:?}", s.get::<String>("sparkpost.token"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
