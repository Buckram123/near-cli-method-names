pub use strum::{EnumDiscriminants, EnumIter, EnumMessage};
pub type CliResult = color_eyre::eyre::Result<()>;

pub async fn online_result(
    connection_config: crate::common::ConnectionConfig,
    block_reference: near_primitives::types::BlockReference,
    contract_id: near_primitives::types::AccountId,
) -> CliResult {
    let client = near_jsonrpc_client::JsonRpcClient::connect(connection_config.rpc_url().as_str());
    let request = near_jsonrpc_client::methods::query::RpcQueryRequest {
        block_reference,
        request: near_primitives::views::QueryRequest::ViewCode {
            account_id: contract_id,
        },
    };
    let status = client.call(request).await.map_err(|err| {
        color_eyre::Report::msg(format!(
            "Failed to fetch query for view contract: {:?}",
            err
        ))
    })?;
    let call_access_view =
        if let near_jsonrpc_primitives::types::query::QueryResponseKind::ViewCode(result) =
            status.kind
        {
            result
        } else {
            return Err(color_eyre::Report::msg("Error call result"));
        };
    for function in wasmer::Module::from_binary(&wasmer::Store::default(), &call_access_view.code)
        .map_err(|err| color_eyre::Report::msg(format!("Not valid wasm file {:?}", err)))?
        .exports()
        .filter(|e| matches!(e.ty(), wasmer::ExternType::Function(_fty)))
    {
        println!("{}", function.name());
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub enum ConnectionConfig {
    Testnet,
    Mainnet,
    Betanet,
    Custom { url: url::Url },
}

pub fn get_account_state(
    connection_config: &ConnectionConfig,
    account_id: near_primitives::types::AccountId,
) -> color_eyre::eyre::Result<Option<near_primitives::views::AccountView>> {
    let query_view_method_response = actix::System::new().block_on(async {
        near_jsonrpc_client::JsonRpcClient::connect(connection_config.rpc_url().as_str())
            .call(near_jsonrpc_client::methods::query::RpcQueryRequest {
                block_reference: near_primitives::types::Finality::Final.into(),
                request: near_primitives::views::QueryRequest::ViewAccount { account_id },
            })
            .await
    });
    match query_view_method_response {
        Ok(rpc_query_response) => {
            let account_view =
                if let near_jsonrpc_primitives::types::query::QueryResponseKind::ViewAccount(
                    result,
                ) = rpc_query_response.kind
                {
                    result
                } else {
                    return Err(color_eyre::Report::msg("Error call result".to_string()));
                };
            Ok(Some(account_view))
        }
        Err(_) => Ok(None),
    }
}

impl ConnectionConfig {
    pub fn from_custom_url(
        custom_url: &crate::types::available_rpc_server_url::AvailableRpcServerUrl,
    ) -> Self {
        Self::Custom {
            url: custom_url.inner.clone(),
        }
    }

    pub fn rpc_url(&self) -> url::Url {
        match self {
            Self::Testnet => crate::consts::TESTNET_API_SERVER_URL.parse().unwrap(),
            Self::Mainnet => crate::consts::MAINNET_API_SERVER_URL.parse().unwrap(),
            Self::Betanet => crate::consts::BETANET_API_SERVER_URL.parse().unwrap(),
            Self::Custom { url } => url.clone(),
        }
    }
}
