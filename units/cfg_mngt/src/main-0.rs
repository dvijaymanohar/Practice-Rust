use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct Server {
    pub debug: bool,
    pub priority: u16,
    pub key: String,
}

fn main() {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("./settings.toml"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    // Read the configuration file
    // match settings.merge(File::with_name("config")) {
    //     Ok(_) => {
    // Access configuration values
    let database_url = settings.get_str("database.url").unwrap();
    let database_port = config.get_int("database.port").unwrap();
    let app_name = config.get_str("app.name").unwrap();
    let app_debug = config.get_bool("app.debug").unwrap();

    // Print the configuration values
    println!("Database URL: {}", database_url);
    println!("Database Port: {}", database_port);
    println!("App Name: {}", app_name);
    println!("App Debug: {}", app_debug);
    //     }
    //     Err(err) => {
    //         eprintln!("Error loading configuration: {}", err);
    //         return;
    //     }
    // }
}
