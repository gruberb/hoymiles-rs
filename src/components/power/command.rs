use clap::{Args, ValueEnum};
use serde::Serialize;

#[derive(Args, Debug, Serialize)]
#[command(
    about = "Fetch your daily power data from a specific solar installation",
    trailing_var_arg = true
)]
#[serde(rename_all = "kebab-case")]
pub struct Power {
    /// Pass the SID of your solar installation
    #[arg(long, env = "SOLAR_SID")]
    pub sid: u32,

    /// Pass the day you want to fetch data for
    #[arg(long, env = "SOLAR_DATE")]
    pub date: String,

    /// Pass the day you want to fetch data for
    #[arg(long, env = "SOLAR_DATE")]
    pub resolution: Resolution,

    /// Save the response in a CSV or JSON file
    #[arg(long, value_enum, env = "SOLAR_SAVE_AS")]
    pub save: Option<FileType>,
}

#[derive(Clone, Debug, ValueEnum, Serialize)]
pub enum FileType {
    Csv,
    Json,
}

#[derive(Clone, Debug, ValueEnum, Serialize, PartialEq, Eq)]
pub enum Resolution {
    Day,
    Week,
    Month,
    Year,
}
