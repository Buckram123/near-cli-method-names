use crate::common::{CliResult, EnumDiscriminants, EnumIter, EnumMessage};

mod block_hash;
mod block_height;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = super::ViewContractMethodsCommandNetworkContext)]
pub struct BlockIdSelector {
    #[interactive_clap(subcommand)]
    pub block_id: BlockId,
}

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = super::ViewContractMethodsCommandNetworkContext)]
pub enum BlockId {
    #[strum_discriminants(strum(message = "View this contract at final block"))]
    /// Specify a block ID final to view this contract
    Final(super::contract_id::CliAccountId),
    #[strum_discriminants(strum(message = "View this contract at block heigt"))]
    /// Specify a block ID height to view this contract
    Height(block_height::BlockIdHeight),
    #[strum_discriminants(strum(message = "View this contract at block hash"))]
    /// Specify a block ID hash to view this contract
    Hash(block_hash::BlockIdHash),
}

impl BlockId {
    pub fn process(self, connection_config: crate::common::ConnectionConfig) {
        match self {
            BlockId::Final(acc) => acc.process(
                connection_config,
                near_primitives::types::BlockReference::Finality(Default::default()),
            ),
            BlockId::Height(height) => height.process(connection_config),
            BlockId::Hash(hash) => hash.process(connection_config),
        }
    }
}
