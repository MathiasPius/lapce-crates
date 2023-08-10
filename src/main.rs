// Deny usage of print and eprint as it won't have same result
// in WASI as if doing in standard program, you must really know
// what you are doing to disable that lint (and you don't know)
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

mod download;
#[macro_use]
mod log;
mod os;
mod version;

use std::str::FromStr;

use anyhow::Result;
use download::crates_lsp;
use lapce_plugin::{
    psp_types::{
        lsp_types::{
            request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, MessageType,
        },
        Request,
    },
    register_plugin, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use os::{Arch, OperatingSystem};
use serde_json::Value;
use version::SelectedVersion;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    let arch = Arch::from_str(&VoltEnvironment::architecture()?)?;
    let os = OperatingSystem::from_str(&VoltEnvironment::operating_system()?)?;

    crate::info!("Initializing lapce-crates plugin",);

    let version = params
        .initialization_options
        .as_ref()
        .and_then(|options| options.get("lsp"))
        .and_then(|lsp| lsp.get("serverPath"))
        .and_then(|server_path| server_path.as_str())
        .unwrap_or_default();

    let version = SelectedVersion::from_str(version)?;

    let lsp = crates_lsp(version, arch, os)?;

    // Target Cargo.toml files specifically.
    let document_selector: DocumentSelector = vec![DocumentFilter {
        language: Some(String::from("toml")),
        pattern: Some(String::from("**/Cargo.toml")),
        scheme: None,
    }];

    crate::info!("Starting crates-lsp server: {lsp:?}");

    PLUGIN_RPC.start_lsp(
        lsp,
        Vec::new(),
        document_selector,
        params.initialization_options,
    )?;

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let _ = PLUGIN_RPC.window_log_message(
                    MessageType::ERROR,
                    format!("CRATES: initialized with {params}"),
                );

                let params: InitializeParams = serde_json::from_value(params).unwrap();

                if let Err(e) = initialize(params) {
                    let _ = PLUGIN_RPC.window_log_message(
                        MessageType::ERROR,
                        format!("CRATES: plugin returned with error: {e}"),
                    );
                }
            }
            _ => {}
        }
    }
}
