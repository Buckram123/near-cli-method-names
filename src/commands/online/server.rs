mod block_id;
mod contract_id;

/// Select server
#[derive(Debug, Clone, strum::EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(strum::EnumMessage, strum::EnumIter))]
pub enum SelectServer {
    /// https://rpc.testnet.near.org
    Testnet(block_id::BlockIdSelector),
    /// https://rpc.mainnet.near.org
    Mainnet(block_id::BlockIdSelector),
    /// https://rpc.betanet.near.org
    Betanet(block_id::BlockIdSelector),
    /// Custom server
    Custom(CustomServer),
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct CustomServer {
    url: crate::types::available_rpc_server_url::AvailableRpcServerUrl,
    #[interactive_clap(subcommand)]
    pub block_id: block_id::BlockId,
}

impl SelectServer {
    pub async fn process(self) {
        let client = near_jsonrpc_client::JsonRpcClient::connect(self.rpc_url().as_str());
        match self {
            SelectServer::Testnet(block)
            | SelectServer::Mainnet(block)
            | SelectServer::Betanet(block) => block.block_id.process(client).await,
            SelectServer::Custom(custom) => custom.block_id.process(client).await,
        }
    }
    pub fn rpc_url(&self) -> url::Url {
        match self {
            Self::Testnet(_) => crate::consts::TESTNET_API_SERVER_URL.parse().unwrap(),
            Self::Mainnet(_) => crate::consts::MAINNET_API_SERVER_URL.parse().unwrap(),
            Self::Betanet(_) => crate::consts::BETANET_API_SERVER_URL.parse().unwrap(),
            Self::Custom(server) => server.url.inner.clone(),
        }
    }
}