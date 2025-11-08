use config_validator::Validate;
use serde::{Deserialize, Serialize};

/// configuration structure
#[derive(
    Serialize, Deserialize, Default, Debug,
    Clone, clap_derive::Args, Validate,
)]
pub struct Settings {
    // optional field
    #[arg(short, long)]
    pub name: Option<String>,
    // optional field
    #[arg(short, long)]
    pub address: Option<String>,
    // mandatory field
    #[arg(short, long)]
    #[validate(mandatory)]
    pub phone_number: Option<String>,
}

#[test]
#[should_panic(expected = r#"[config-validator] mandatory field "phone_number" not provided for struct "Settings""#)]
fn uninitialized_field_test() {
    let s = Settings::default();
    let _ = s.validate_configuration();
}

#[test]
fn field_getter_test() -> anyhow::Result<()> {
    let s = Settings {
        phone_number: Some(String::from("555-1234")),
        ..Default::default()
    };
    let s: ValidatedSettings = s.validate_configuration();
    let _: String = s.get_phone_number();

    Ok(())
}
