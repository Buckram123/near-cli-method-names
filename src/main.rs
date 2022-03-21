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
    Online(commands::Online),
    #[strum_discriminants(strum(message = "From wasm file"))]
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
