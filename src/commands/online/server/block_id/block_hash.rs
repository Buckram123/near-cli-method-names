#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = super::super::ViewContractMethodsCommandNetworkContext)]
pub struct BlockIdHash {
    block_id_hash: crate::types::crypto_hash::CryptoHash,
    #[interactive_clap(named_arg)]
    contract_id: super::super::contract_id::CliAccountId,
}

impl BlockIdHash {
    pub async fn process(self, connection_config: crate::common::ConnectionConfig) {
        self.contract_id
            .process(
                connection_config,
                near_primitives::types::BlockReference::BlockId(
                    near_primitives::types::BlockId::Hash(self.block_id_hash.into()),
                ),
            )
            .await;
    }
}
