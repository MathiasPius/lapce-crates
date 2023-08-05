// Deny usage of print and eprint as it won't have same result
// in WASI as if doing in standard program, you must really know
// what you are doing to disable that lint (and you don't know)
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

mod os;

use std::{io::Cursor, path::PathBuf, str::FromStr};

use anyhow::{anyhow, Result};
use flate2::bufread::GzDecoder;
use lapce_plugin::{
    psp_types::{
        lsp_types::{
            request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, MessageType,
            Url,
        },
        Request,
    },
    register_plugin, Http, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use os::{Arch, OperatingSystem};
use serde_json::Value;
use tar::Archive;
use zip::ZipArchive;

#[derive(Default)]
struct State {}

register_plugin!(State);

const LSP_VERSION: &str = "0.0.1";

fn download_crates_lsp() -> Result<Url> {
    let arch = Arch::from_str(&VoltEnvironment::architecture()?)?;
    let os = OperatingSystem::from_str(&VoltEnvironment::operating_system()?)?;

    let archive = format!(
        "crates-lsp-{arch}-{target}.{ext}",
        target = os.as_target(),
        ext = os.archive_format()
    );

    // Download URL
    let uri = format!(
        "https://github.com/MathiasPius/crates-lsp/releases/download/v{LSP_VERSION}/{archive}"
    );

    let volt_uri = VoltEnvironment::uri()?;

    PLUGIN_RPC.window_log_message(
        MessageType::INFO,
        format!("downloading {uri} into {volt_uri}"),
    )?;
    let mut resp = Http::get(&uri)?;
    let body = resp.body_read_all()?;

    PLUGIN_RPC.window_log_message(
        MessageType::INFO,
        format!("PATH: {:#?}", std::env::current_dir()),
    )?;

    let mut file = std::fs::File::create(PathBuf::from("/").join(os.executable()))?;

    PLUGIN_RPC.window_log_message(MessageType::INFO, "Unpacking".to_string())?;

    // Extract the contained executable.
    match os.archive_format() {
        os::ArchiveFormat::Zip => {
            let mut archive = ZipArchive::new(Cursor::new(body))?;
            let mut reader = archive.by_name(os.executable())?;
            std::io::copy(&mut reader, &mut file)?;
        }
        os::ArchiveFormat::TarGz => {
            let mut archive = Archive::new(GzDecoder::new(Cursor::new(body)));
            let mut reader = archive.entries()?.next().unwrap()?;
            std::io::copy(&mut reader, &mut file)?;
        }
    }

    Ok(Url::parse(&volt_uri)?.join(os.executable())?)
}

fn initialize(params: InitializeParams) -> Result<()> {
    PLUGIN_RPC
        .window_log_message(
            MessageType::INFO,
            "Initializing lapce-crates plugin".to_string(),
        )
        .unwrap();

    /*

    */

    let server_uri = if let Some(server_path) = params
        .initialization_options
        .as_ref()
        .and_then(|options| options.get("lsp"))
        .and_then(|lsp| lsp.get("serverPath"))
        .and_then(|server_path| server_path.as_str())
        .filter(|server_path| !server_path.is_empty())
    {
        Url::parse(&format!("urn:{}", server_path))?
    } else {
        /*
        PLUGIN_RPC.window_show_message(
            MessageType::INFO,
            "Could not load Crates plugin.Please manually install the 'crates-lsp' binary, and specify the path to it in the plugin settings. See lapce-crates plugin README for guidance.".to_string()
        )?;
        */

        download_crates_lsp()?

        //return Err(anyhow!("Could not load Crates plugin. Please manually install the 'crates-lsp' binary, and specify the path to it in the plugin settings."));
    };

    // Target Cargo.toml files specifically.
    let document_selector: DocumentSelector = vec![DocumentFilter {
        language: Some(String::from("toml")),
        pattern: Some(String::from("**/Cargo.toml")),
        scheme: None,
    }];

    PLUGIN_RPC.window_log_message(
        MessageType::INFO,
        format!("Starting server: {server_uri:#?}"),
    )?;

    PLUGIN_RPC.start_lsp(
        server_uri,
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
