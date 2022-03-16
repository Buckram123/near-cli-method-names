#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct BlockIdHash {
    block_id_hash: crate::types::crypto_hash::CryptoHash,
    contract_id: crate::types::account_id::AccountId,
}

impl BlockIdHash {
    pub async fn process(
        self,
        client: near_jsonrpc_client::JsonRpcClient<near_jsonrpc_client::auth::Unauthenticated>,
    ) {
        crate::common::online_result(
            client,
            near_primitives::types::BlockReference::BlockId(near_primitives::types::BlockId::Hash(
                self.block_id_hash.into(),
            )),
            self.contract_id.into(),
        )
        .await;
    }
}
