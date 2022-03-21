#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = super::super::SelectServerContext)]
pub struct BlockIdHeight {
    block_id_height: near_primitives::types::BlockHeight,
    #[interactive_clap(named_arg)]
    contract_id: super::super::contract_id::CliAccountId,
}

impl BlockIdHeight {
    pub async fn process(
        self,
        client: near_jsonrpc_client::JsonRpcClient<near_jsonrpc_client::auth::Unauthenticated>,
    ) {
        self.contract_id
            .process(
                client,
                near_primitives::types::BlockReference::BlockId(
                    near_primitives::types::BlockId::Height(self.block_id_height),
                ),
            )
            .await;
    }
}
