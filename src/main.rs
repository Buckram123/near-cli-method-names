mod commands;
mod common;
mod consts;
mod types;

use common::{CliResult, EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = ())]
struct Args {
    #[interactive_clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
/// Choose from where you want to check functions of contract
pub enum Commands {
    #[strum_discriminants(strum(message = "From blockchain"))]
    /// Blockchain
    Blockchain(commands::Blockchain),
    #[strum_discriminants(strum(message = "From wasm file"))]
    /// Wasm file of contract
    Wasm(commands::Wasm),
}

impl Commands {
    fn process(self) -> CliResult {
        match self {
            Commands::Blockchain(val) => val.process(),
            Commands::Wasm(val) => val.process(),
        }
    }
}
fn main() -> CliResult {
    let cli = Args::parse();
    let args = Args::from_cli(Some(cli), ())?;

    args.command.process()
}
