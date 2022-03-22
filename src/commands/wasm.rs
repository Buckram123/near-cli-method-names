use crate::common::CliResult;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct Wasm {
    /// What is a file location of the contract?
    path: crate::types::path_buf::PathBuf,
}

impl Wasm {
    pub fn process(self) -> CliResult {
        for function in wasmer::Module::from_file(&wasmer::Store::default(), self.path.0)
            .map_err(|err| color_eyre::Report::msg(format!("Not valid wasm file {:?}", err)))?
            .exports()
            .filter(|e| matches!(e.ty(), wasmer::ExternType::Function(_)))
        {
            println!("{}", function.name());
        }
        Ok(())
    }
}
