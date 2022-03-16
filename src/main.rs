mod commands;
mod common;
mod consts;
mod types;

use common::*;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = ())]
struct Args {
    #[interactive_clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
pub enum Commands {
    /// Online
    Online(commands::Online),
    /// Wasm file of contract
    Wasm(commands::Wasm),
}

impl Commands {
    async fn process(self) {
        match self {
            Commands::Online(val) => val.process().await,
            Commands::Wasm(val) => val.process(),
        }
    }
}
fn main() -> CliResult {
    let cli = Args::parse();
    let args = Args::from_cli(Some(cli), ())?;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let process_result = actix::System::new().block_on(args.command.process());
    Ok(process_result)
}
