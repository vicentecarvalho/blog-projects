use clap::ArgMatches;
use config::{Source, Value, ValueKind};
use std::collections::HashMap;

/// A wrapper for `clap::ArgMatches`
#[derive(Debug, Clone)]
pub struct ClapSource {
    matches: ArgMatches,
}

impl ClapSource {
    /// Wraps `ArgMatches` as a ClapSource object, allowing the usage
    /// of this as a `Config` source later on
    pub fn new(matches: &ArgMatches) -> Self {
        ClapSource { matches: matches.to_owned() }
    }
}

/// Implements the `config::Source` trait for ClapSource
impl Source for ClapSource {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(
        &self,
    ) -> std::result::Result<
        HashMap<std::string::String, config::Value>,
        config::ConfigError,
    > {
        let mut config_map = HashMap::new();
        let origin: String = "cli".into();

        for id in self.matches.ids() {
            if let Some(value_kind) = self.extract_value(id.as_str()) {
                config_map.insert(
                    id.to_string(),
                    Value::new(Some(&origin), value_kind),
                );
            }
        }

        Ok(config_map)
    }
}

impl ClapSource {
    fn extract_value(&self, id_str: &str) -> Option<ValueKind> {
        // Try each type in order, returning early on success
        if let Ok(Some(val)) = self.matches.try_get_one::<bool>(id_str) {
            return Some(ValueKind::Boolean(*val));
        }

        if let Ok(Some(val)) = self.matches.try_get_one::<i64>(id_str) {
            return Some(ValueKind::I64(*val));
        }

        if let Ok(Some(val)) = self.matches.try_get_one::<f64>(id_str) {
            return Some(ValueKind::Float(*val));
        }

        if let Ok(Some(val)) = self.matches.try_get_one::<String>(id_str) {
            return Some(ValueKind::String(val.clone()));
        }

        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use clap::Args;
    use clap::error::ErrorKind::MissingRequiredArgument;
    use config::Config;
    use serde::Deserialize;

    /// configuration structure
    #[derive(Deserialize, Debug, Clone, clap_derive::Args)]
    pub struct Settings {
        // optional field
        #[arg(short, long)]
        pub name: Option<String>,
        // optional field
        #[arg(short, long)]
        pub address: Option<String>,
        // mandatory field
        #[arg(short, long)]
        pub phone_number: String,
    }

    #[test]
    pub fn clap_config_source() {
        let cli = clap::Command::new("test");
        let cli = Settings::augment_args(cli);

        let matches = cli
            .try_get_matches_from([
                "test",
                "--name=Joe",
                "--phone-number=555-1234",
            ])
            .unwrap();

        let clap_source = ClapSource::new(&matches);

        let c = Config::builder().add_source(clap_source).build().unwrap();

        let s: Settings = c.try_deserialize().unwrap();

        assert_eq!(s.name, Some(String::from("Joe")));
        assert!(s.address.is_none());
        assert_eq!(s.phone_number, "555-1234");
    }

    #[test]
    pub fn clap_mandatory_arg() {
        let cli = clap::Command::new("test");
        let cli = Settings::augment_args(cli);

        let result = cli.try_get_matches_from(["test", "--name=Joe"]);
        
        assert!(result.is_err());
        
        if let Err(err) = result {
            assert_eq!(err.kind(), MissingRequiredArgument);

            // Check that the error message mentions the missing field
            assert!(err.to_string().contains("--phone-number"));
        }
    }
}
