mod block_hash;
mod block_height;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct BlockIdSelector {
    #[interactive_clap(subcommand)]
    pub block_id: BlockId,
}

#[derive(Debug, Clone, strum::EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(strum::EnumMessage, strum::EnumIter))]
pub enum BlockId {
    Final(super::contract_id::CliAccountId),
    Height(block_height::BlockIdHeight),
    Hash(block_hash::BlockIdHash),
}

impl BlockId {
    pub async fn process(self, client: near_jsonrpc_client::JsonRpcClient<near_jsonrpc_client::auth::Unauthenticated>) {
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
