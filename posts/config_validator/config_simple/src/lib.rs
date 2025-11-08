use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG_FILE_PATH: &str = "config.yaml";

/// configuration structure
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Settings {
    // optional field
    pub name: Option<String>,
    // optional field
    pub address: Option<String>,
    // mandatory field
    pub phone_number: String,
}

#[allow(dead_code)]
fn run_configuration() -> Settings {
    // app configuration
    let c = Config::builder()
        .add_source(File::with_name(DEFAULT_CONFIG_FILE_PATH).required(false))
        .add_source(Environment::default())
        .build()
        .unwrap();

    c.try_deserialize().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    //
    // default config file only
    //
    #[test]
    fn default_config_test() -> () {
        let s = run_configuration();

        assert!(s.name.is_some());
        assert!(s.address == Some("123 Main St".to_owned()));
        assert!(s.phone_number == "555-1234");
    }

    //
    // env var overwrite
    //
    #[test]
    fn env_var_overwrite_test() -> () {
        env::set_var("NAME", "Jack");

        let s = run_configuration();

        assert!(s.name == Some(String::from("Jack")));
        assert!(s.address == Some("123 Main St".to_owned()));
        assert!(s.phone_number == "555-1234");
    }
}
