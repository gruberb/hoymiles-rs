use clap::{Parser, Subcommand};

pub(crate) mod components;
pub(crate) mod config;
pub(crate) mod routes;

use tracing_log::LogTracer;

#[derive(Parser, Debug)]
#[clap(
    name = "hoymiles",
    about = "A CLI to interact with your HoyMiles solar installation"
)]
pub(crate) struct Opt {
    /// Defines the verbosity level
    #[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
        global = true
    )]
    pub(crate) verbose: u8,

    #[command(subcommand)]
    pub(crate) commands: HoyMilesCommand,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    LogTracer::init()?;

    let args = Opt::parse();

    match args.commands {
        HoyMilesCommand::Login(cmd) => components::login::handle_command(cmd).await,
        HoyMilesCommand::Power(cmd) => components::power::handle_command(cmd).await,
    }

    std::process::exit(0);
}

#[derive(Subcommand, Debug)]
pub(crate) enum HoyMilesCommand {
    Login(components::login::command::Login),
    Power(components::power::command::Power),
}
