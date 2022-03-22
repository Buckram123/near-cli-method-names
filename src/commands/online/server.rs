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
pub struct ViewContractMethodsCommandNetworkContext {
    pub connection_config: crate::common::ConnectionConfig,
}

impl From<SelectServerContext> for ViewContractMethodsCommandNetworkContext {
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
#[interactive_clap(input_context = SelectServerContext)]
#[interactive_clap(output_context = ViewContractMethodsCommandNetworkContext)]
pub struct CustomServer {
    #[interactive_clap(long)]
    /// What is the RPC endpoint?
    pub url: crate::types::available_rpc_server_url::AvailableRpcServerUrl,
    #[interactive_clap(subcommand)]
    pub block_id: block_id::BlockId,
}

struct CustomServerContext {
    pub url: crate::types::available_rpc_server_url::AvailableRpcServerUrl,
}

impl CustomServerContext {
    fn from_previous_context(
        _previous_context: SelectServerContext,
        scope: &<CustomServer as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {
            url: scope.url.clone(),
        }
    }
}

impl From<CustomServerContext> for ViewContractMethodsCommandNetworkContext {
    fn from(item: CustomServerContext) -> Self {
        Self {
            connection_config: crate::common::ConnectionConfig::from_custom_url(&item.url),
        }
    }
}

impl SelectServer {
    pub fn process(self) {
        match self {
            SelectServer::Testnet(block) => block
                .block_id
                .process(crate::common::ConnectionConfig::Testnet),
            SelectServer::Mainnet(block) => block
                .block_id
                .process(crate::common::ConnectionConfig::Mainnet),
            SelectServer::Betanet(block) => block
                .block_id
                .process(crate::common::ConnectionConfig::Betanet),
            SelectServer::Custom(custom) => {
                custom
                    .block_id
                    .process(crate::common::ConnectionConfig::Custom {
                        url: custom.url.inner,
                    })
            }
        }
    }
}
