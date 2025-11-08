mod settings;

use crate::settings::{CliCommands, ValidatedSettings};
use anyhow::{Result, anyhow};
use log::info;
use settings::Settings;
use std::str::FromStr;

fn my_function(settings: ValidatedSettings) {
    info!("config_file: {:?}", settings.get_config_file());
    info!("log_config_file: {:?}", settings.get_log_config_file());
    info!("address: {:?}", settings.get_address());
    info!("name: {:?}", settings.get_name());
    info!("phone_number: {:?}", settings.get_phone_number());
}

fn main() -> Result<()> {
    // load settings
    let (settings, matches) = Settings::process()?;

    // load log configuration
    log4rs::init_file(settings.get_log_config_file(), Default::default())
        .map_err(|e| anyhow!("[log_config] {e}"))?;

    let Some((command, _)) = matches.subcommand() else {
        return Err(anyhow!("no command provided"));
    };

    let cli_command = CliCommands::from_str(command)?;

    match cli_command {
        CliCommands::Start => {
            info!("command start");
            my_function(settings);
        }
        CliCommands::SayHello => info!("Hello User"),
    }

    Ok(())
}
