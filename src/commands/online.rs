use crate::common::CliResult;

mod server;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct Online {
    #[interactive_clap(subcommand)]
    server: server::SelectServer,
}

impl Online {
    pub fn process(self) -> CliResult {
        self.server.process()
    }
}
