mod server;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct Online {
    #[interactive_clap(subcommand)]
    server: server::SelectServer,
}

impl Online {
    pub async fn process(self) {
        self.server.process().await;
    }
}
