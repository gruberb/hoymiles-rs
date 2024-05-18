use clap::Args;
use serde::Serialize;

#[derive(Args, Debug, Serialize)]
#[command(
    about = "Fetch your daily power data from a specific solar installation",
    trailing_var_arg = true
)]
#[serde(rename_all = "kebab-case")]
pub struct Power {
    /// Pass the SSID of your solar installation
    #[arg(long, env = "SOLAR_SSID")]
    pub ssid: u32,

    /// Pass the day you want to fetch data for
    #[arg(long, value_enum, env = "SOLAR_DATE")]
    pub date: String,
}
