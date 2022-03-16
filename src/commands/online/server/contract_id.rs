#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct CliAccountId {
    contract_id: crate::types::account_id::AccountId,
}

impl CliAccountId {
    pub async fn process(
        self,
        client: near_jsonrpc_client::JsonRpcClient<near_jsonrpc_client::auth::Unauthenticated>,
        block_reference: near_primitives::types::BlockReference,
    ) {
        crate::common::online_result(client, block_reference, self.contract_id.into()).await
    }
}
