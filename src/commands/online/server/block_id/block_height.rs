use crate::common::CliResult;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = super::super::ViewContractMethodsCommandNetworkContext)]
pub struct BlockIdHeight {
    block_id_height: near_primitives::types::BlockHeight,
    #[interactive_clap(named_arg)]
    contract_id: super::super::contract_id::CliAccountId,
}

impl BlockIdHeight {
    pub fn process(self, connection_config: crate::common::ConnectionConfig) -> CliResult {
        self.contract_id.process(
            connection_config,
            near_primitives::types::BlockReference::BlockId(
                near_primitives::types::BlockId::Height(self.block_id_height),
            ),
        )
    }
}
