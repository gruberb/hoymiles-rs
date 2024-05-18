use clap::Args;
use serde::Serialize;

#[derive(Args, Debug, Serialize)]
#[command(about = "Login to your Hoymiles account", trailing_var_arg = true)]
#[serde(rename_all = "kebab-case")]
pub struct Login {
    /// Username you use for your HoyMiles account
    #[arg(long, env = "SOLAR_USER_NAME")]
    pub user_name: String,

    /// Password you use for your HoyMiles account
    #[arg(long, value_enum, env = "SOLAR_PASSWORD")]
    pub password: String,
}
