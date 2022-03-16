#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct BlockIdHeight {
    block_id_height: near_primitives::types::BlockHeight,
    contract_id: crate::types::account_id::AccountId,
}

impl BlockIdHeight {
    pub async fn process(self, client: near_jsonrpc_client::JsonRpcClient<near_jsonrpc_client::auth::Unauthenticated>) {
        crate::common::online_result(
            client,
            near_primitives::types::BlockReference::BlockId(
                near_primitives::types::BlockId::Height(self.block_id_height),
            ),
            self.contract_id.into(),
        )
        .await;
    }
}
