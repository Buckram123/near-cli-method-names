use crate::common::{CliResult, EnumDiscriminants, EnumIter, EnumMessage};
mod block_id;
mod contract_id;

/// Select server
#[derive(Debug, Clone, EnumDiscriminants, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = SelectServerContext)]
/// Select NEAR protocol RPC server
pub enum SelectServer {
    /// Provide data for the server https://rpc.testnet.near.org
    #[strum_discriminants(strum(message = "Testnet"))]
    Testnet(block_id::BlockIdSelector),
    /// Provide data for the server https://rpc.mainnet.near.org
    #[strum_discriminants(strum(message = "Mainnet"))]
    Mainnet(block_id::BlockIdSelector),
    /// Provide data for the server https://rpc.betanet.near.org
    #[strum_discriminants(strum(message = "Betanet"))]
    Betanet(block_id::BlockIdSelector),
    /// Provide data for a manually specified server
    #[strum_discriminants(strum(message = "Custom"))]
    Custom(CustomServer),
}

#[derive(Clone)]
pub struct SelectServerContext {
    selected_server: SelectServerDiscriminants,
}

impl SelectServerContext {
    fn from_previous_context(
        _previous_context: (),
        scope: &<SelectServer as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {
            selected_server: *scope,
        }
    }
}

#[derive(Clone)]
pub struct ViewContractCodeCommandNetworkContext {
    pub connection_config: crate::common::ConnectionConfig,
}

impl From<SelectServerContext> for ViewContractCodeCommandNetworkContext {
    fn from(item: SelectServerContext) -> Self {
        let connection_config = match item.selected_server {
            SelectServerDiscriminants::Testnet => crate::common::ConnectionConfig::Testnet,
            SelectServerDiscriminants::Mainnet => crate::common::ConnectionConfig::Mainnet,
            SelectServerDiscriminants::Betanet => crate::common::ConnectionConfig::Betanet,
            SelectServerDiscriminants::Custom => {
                unreachable!("Network context should not be constructed from Custom variant")
            }
        };
        Self { connection_config }
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = SelectServerContext)]
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
