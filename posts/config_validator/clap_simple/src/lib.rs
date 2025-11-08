use clap::{ArgMatches, Args, FromArgMatches, Subcommand};
use clap_derive;
use strum_macros::EnumString;

/// configuration structure
#[derive(Debug, Clone, clap_derive::Args)]
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

/// List available cli commands
#[derive(Debug, clap_derive::Subcommand, PartialEq, EnumString)]
pub enum CliCommands {
    // allow commands to be sent to the application
    #[strum(serialize = "start")]
    #[command(about = "Starts the application")]
    Start,
}

#[allow(dead_code)]
fn run_configuration(arg_vec: Vec<&str>) -> (Settings, ArgMatches) {
    let cli = clap::Command::new("clap-simple");
    let cli = Settings::augment_args(cli);
    let cli = CliCommands::augment_subcommands(cli);
    let mut matches = cli.get_matches_from(arg_vec);

    // Convert ArgMatches -> GreetArgs (FromArgMatches is derived)
    let settings = Settings::from_arg_matches_mut(&mut matches)
        .expect("error parsing arguments into Settings");

    (settings, matches)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn default_config_test() -> () {
        let arg_vec = vec![
            "clap-simple",
            "-n",
            "Jack",
            "--phone-number",
            "555-1234",
            "start",
        ];
        let (settings, matches) = run_configuration(arg_vec);

        assert!(settings.name == Some(String::from("Jack")));
        assert!(settings.address.is_none());
        assert!(settings.phone_number == "555-1234");

        let (command, _args) = matches.subcommand().unwrap();
        let cli_command = CliCommands::from_str(command).unwrap();

        assert!(matches!(cli_command, CliCommands::Start));
    }

    #[test]
    fn help_menu() -> () {
        let arg_vec = vec!["clap-simple", "-h"];
        let _ = run_configuration(arg_vec);
    }
}
