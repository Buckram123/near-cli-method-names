#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct Wasm {
    /// What is a file location of the contract?
    path: crate::types::path_buf::PathBuf,
}

impl Wasm {
    pub fn process(self) {
        for function in wasmer::Module::from_file(&wasmer::Store::default(), self.path.0)
            .unwrap()
            .exports()
            .filter(|e| matches!(e.ty(), wasmer::ExternType::Function(_)))
        {
            println!("{}", function.name());
        }
    }
}
