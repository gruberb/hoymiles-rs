use clap::Args;
use serde::Serialize;

#[derive(Args, Debug, Serialize)]
#[command(
    about = "Create daily reports from your whole solar installation",
    trailing_var_arg = true
)]
#[serde(rename_all = "kebab-case")]
pub struct Report {
    /// Pass the SIDs of your solar installation
    #[arg(long, env = "SOLAR_SIDS", value_delimiter = ',')]
    pub sids: Vec<u32>,
}
