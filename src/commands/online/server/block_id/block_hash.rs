#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = super::super::SelectServerContext)]
pub struct BlockIdHash {
    block_id_hash: crate::types::crypto_hash::CryptoHash,
    #[interactive_clap(named_arg)]
    contract_id: super::super::contract_id::CliAccountId,
}

impl BlockIdHash {
    pub async fn process(
        self,
        client: near_jsonrpc_client::JsonRpcClient<near_jsonrpc_client::auth::Unauthenticated>,
    ) {
        self.contract_id
            .process(
                client,
                near_primitives::types::BlockReference::BlockId(
                    near_primitives::types::BlockId::Hash(self.block_id_hash.into()),
                ),
            )
            .await;
    }
}
