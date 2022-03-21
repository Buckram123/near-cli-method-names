use dialoguer::Input;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = super::SelectServerContext)]
#[interactive_clap(output_context = ())]
pub struct CliAccountId {
    #[interactive_clap(skip_default_from_cli_arg)]
    contract_id: crate::types::account_id::AccountId,
}

impl CliAccountId {
    fn from_cli_contract_id(
        optional_cli_sender_account_id: Option<crate::types::account_id::AccountId>,
        context: &super::SelectServerContext,
    ) -> color_eyre::eyre::Result<crate::types::account_id::AccountId> {
        let context: super::ViewContractCodeCommandNetworkContext = context.clone().into();
        match optional_cli_sender_account_id {
            Some(cli_sender_account_id) => match crate::common::get_account_state(
                &context.connection_config,
                cli_sender_account_id.clone().into(),
            )? {
                Some(_) => Ok(cli_sender_account_id),
                None => {
                    println!("Account <{}> doesn't exist", cli_sender_account_id);
                    Self::input_contract_account_id(&context)
                }
            },
            None => Self::input_contract_account_id(&context),
        }
    }
    
    pub async fn process(
        self,
        client: near_jsonrpc_client::JsonRpcClient<near_jsonrpc_client::auth::Unauthenticated>,
        block_reference: near_primitives::types::BlockReference,
    ) {
        crate::common::online_result(client, block_reference, self.contract_id.into()).await
    }

    pub fn input_contract_account_id(
        context: &super::ViewContractCodeCommandNetworkContext,
    ) -> color_eyre::eyre::Result<crate::types::account_id::AccountId> {
        loop {
            let account_id: crate::types::account_id::AccountId = Input::new()
                .with_prompt("What contract do you need to view?")
                .interact_text()?;
            if (crate::common::get_account_state(
                &context.connection_config,
                account_id.clone().into(),
            )?).is_some() {
                break Ok(account_id);
            } else {
                println!("Account <{}> doesn't exist", account_id);
            };
        }
    }

}
