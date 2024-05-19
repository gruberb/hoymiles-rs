use clap::{Args, ValueEnum};
use serde::Serialize;

#[derive(Args, Debug, Serialize)]
#[command(
    about = "Fetch your daily power data from a specific solar installation",
    trailing_var_arg = true
)]
#[serde(rename_all = "kebab-case")]
pub struct Compare {
    /// Pass the SIDs of your solar installation
    #[arg(long, env = "SOLAR_SIDS")]
    pub sids: Vec<u32>,

    /// Pass the day you want to fetch data for
    #[arg(long, env = "SOLAR_DATE")]
    pub timeframe: TimeFrame,

    /// Save the response in a CSV or JSON file
    #[arg(long, value_enum, env = "SOLAR_SAVE_AS")]
    pub save: Option<FileType>,
}

#[derive(Clone, Debug, ValueEnum, Serialize)]
pub enum TimeFrame {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}
