use anyhow::Result;
use clap::{ArgMatches, Args, Subcommand};
use clap_config_source::ClapSource;
use config::{Config, Environment, File};
use config_validator::Validate;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

const DEFAULT_LOG_CONFIG: &str = "files/log_config.yaml";
const DEFAULT_APP_CONFIG: &str = "files/app_config.yaml";

#[derive(clap_derive::Subcommand, Debug)]
enum Commands {}

#[derive(
    Serialize, Deserialize, Debug, Clone, 
    Default, clap_derive::Args, Validate
)]
pub struct Settings {
    #[arg(short, long, default_value = &DEFAULT_APP_CONFIG)]
    #[validate(mandatory)]
    pub config_file: Option<String>,
    #[arg(short, long)]
    #[validate(mandatory)]
    pub log_config_file: Option<String>,
    #[arg(short, long)]
    pub name: Option<String>,
    #[arg(short, long)]
    pub address: Option<String>,
    #[arg(short, long)]
    #[validate(mandatory)]
    pub phone_number: Option<String>,
}

/// List available cli commands
#[derive(Debug, clap_derive::Subcommand, PartialEq, EnumString)]
pub enum CliCommands {
    #[strum(serialize = "start")]
    #[command(about = "Starts the application")]
    Start,
    #[strum(serialize = "say-hello")]
    #[command(about = "Say Hello back to user")]
    SayHello,
}

impl Settings {
    /// Instantiate a Settings object
    /// * Parse the passed configuration file
    /// * Read environment variables
    /// * Read cli
    ///
    /// ### Returns:
    /// A Validated instance of this Settings and ArgMatches
    pub fn process() -> Result<(ValidatedSettings, ArgMatches)> {
        // cli input
        let cli = clap::Command::new("my-app");
        let cli = Settings::augment_args(cli);
        let cli = CliCommands::augment_subcommands(cli);
        let matches = cli.get_matches();

        // app configuration
        let clap_source = ClapSource::new(&matches);
        let c = Config::builder()
            .add_source(File::with_name(&DEFAULT_APP_CONFIG).required(false))
            .add_source(Environment::default())
            .add_source(clap_source)
            .set_default("log_config_file", DEFAULT_LOG_CONFIG)?
            .build()
            .unwrap();

        let s: Settings = c.try_deserialize().unwrap();
        let s: ValidatedSettings = s.validate_configuration();

        Ok((s, matches))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_test() -> Result<()> {
        let c = Config::builder()
            .add_source(File::with_name("cfg/config.yaml").required(false))
            .set_default("log_config_file", DEFAULT_LOG_CONFIG)?
            .build()
            .unwrap();

        let _: Settings = c.try_deserialize()?;

        Ok(())
    }
}
