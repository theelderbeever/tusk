// mod tune;
mod config;
mod queries;
mod types;

use clap::{Parser, Subcommand};
use config::Config;
// use sysinfo::{Components, Disks, Networks, System};

// use tune::TuneCommand;
use queries::QueryCommand;

fn print_version() -> &'static str {
    Box::leak(format!("v{}", env!("CARGO_PKG_VERSION")).into())
}

#[derive(Debug, Parser)]
#[command(name = "tusk")]
#[command(version = print_version(), about = "Postgres tuning and utility cli", long_about = None)]
pub struct Cli {
    #[arg(
        short,
        long,
        help = "Connection profile to use. Will use `default` in ~/.tusk/config.toml"
    )]
    pub profile: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Ls,
    // Tune(TuneCommand),
    Query(QueryCommand),
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();

    let config = Config::read_config();

    match cli.command {
        Commands::Ls => config.list_connection_profiles(),
        Commands::Query(cmd) => cmd.exec(config.get(cli.profile)).await,
    };
}
