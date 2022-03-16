#[derive(Debug, Clone, PartialEq)]
pub struct AvailableRpcServerUrl {
    pub inner: url::Url,
}

impl std::str::FromStr for AvailableRpcServerUrl {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url: url::Url =
            url::Url::parse(s).map_err(|err| format!("URL is not parsed: {}", err))?;
        actix::System::new()
            .block_on(async {
                near_jsonrpc_client::JsonRpcClient::connect(&url.as_str())
                    .call(near_jsonrpc_client::methods::status::RpcStatusRequest)
                    .await
            })
            .map_err(|err| format!("AvailableRpcServerUrl: {:?}", err))?;
        Ok(Self { inner: url })
    }
}

impl std::fmt::Display for AvailableRpcServerUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl interactive_clap::ToCli for AvailableRpcServerUrl {
    type CliVariant = AvailableRpcServerUrl;
}
