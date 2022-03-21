use crate::common::{CliResult, EnumDiscriminants, EnumIter, EnumMessage};

mod block_hash;
mod block_height;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = super::SelectServerContext)]
pub struct BlockIdSelector {
    #[interactive_clap(subcommand)]
    pub block_id: BlockId,
}

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = super::SelectServerContext)]
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
    pub async fn process(
        self,
        client: near_jsonrpc_client::JsonRpcClient<near_jsonrpc_client::auth::Unauthenticated>,
    ) {
        match self {
            BlockId::Final(acc) => {
                acc.process(
                    client,
                    near_primitives::types::BlockReference::Finality(Default::default()),
                )
                .await
            }
            BlockId::Height(height) => height.process(client).await,
            BlockId::Hash(hash) => hash.process(client).await,
        }
    }
}
