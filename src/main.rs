// Deny usage of print and eprint as it won't have same result
// in WASI as if doing in standard program, you must really know
// what you are doing to disable that lint (and you don't know)
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

//mod os;

use anyhow::{anyhow, Result};
use lapce_plugin::{
    psp_types::{
        lsp_types::{
            request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, MessageType,
            Url,
        },
        Request,
    },
    register_plugin, LapcePlugin, PLUGIN_RPC,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

//const LSP_VERSION: &str = "0.0.1";

fn initialize(params: InitializeParams) -> Result<()> {
    PLUGIN_RPC
        .window_log_message(
            MessageType::INFO,
            "Initializing lapce-crates plugin".to_string(),
        )
        .unwrap();

    /*
    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(lsp) = options.get("lsp") {
            if let Some(args) = lsp.get("serverArgs") {
                if let Some(args) = args.as_array() {
                    if !args.is_empty() {
                        server_args = vec![];
                    }
                    for arg in args {
                        if let Some(arg) = arg.as_str() {
                            server_args.push(arg.to_string());
                        }
                    }
                }
            }

            if let Some(server_path) = lsp.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        let server_uri = Url::parse(&format!("urn:{}", server_path))?;
                        let _ = PLUGIN_RPC.start_lsp(
                            server_uri,
                            server_args,
                            document_selector,
                            params.initialization_options,
                        );
                        return Ok(());
                    }
                }
            }
        }
    }
    */

    /*
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

    PLUGIN_RPC.window_log_message(
            MessageType::INFO,
            format!("Unpacking"),
        )?;

    // Extract the contained executable.
    match os.archive_format() {
        os::ArchiveFormat::Zip => {
            let mut archive = ZipArchive::new(Cursor::new(body))?;
            let mut reader = archive.by_name(os.executable())?;
            std::io::copy(&mut reader, &mut file)?;
        }
        os::ArchiveFormat::TarGz => {
            let mut reader = GzDecoder::new(Cursor::new(body));
            std::io::copy(&mut reader, &mut file)?;
        }
    }


    let server_uri = Url::parse(&volt_uri)?.join(os.executable())?;
    */

    let Some(server_path) = params
        .initialization_options
        .as_ref()
        .and_then(|options| options.get("lsp"))
        .and_then(|lsp| lsp.get("serverPath"))
        .and_then(|server_path| server_path.as_str())
    else {
        PLUGIN_RPC.window_show_message(
            MessageType::ERROR, 
            "Could not load Crates plugin.Please manually install the 'crates-lsp' binary, and specify the path to it in the plugin settings. See lapce-crates plugin README for guidance.".to_string()
        )?;

        return Err(anyhow!("Could not load Crates plugin. Please manually install the 'crates-lsp' binary, and specify the path to it in the plugin settings."));
    };

    let server_uri = Url::parse(&format!("urn:{}", server_path))?;

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
