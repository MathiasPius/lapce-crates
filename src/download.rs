use std::{
    io::{Cursor, Write},
    path::Path,
};

use flate2::bufread::GzDecoder;
use lapce_plugin::{psp_types::lsp_types::Url, Http, VoltEnvironment};
use semver::Version;
use tar::Archive;
use zip::ZipArchive;

use crate::{
    os::{Arch, ArchiveFormat, OperatingSystem},
    version::SelectedVersion,
};

const LATEST_VERSION: Version = Version::new(0, 0, 3);

fn download_into(
    mut file: impl Write,
    version: &Version,
    arch: Arch,
    os: OperatingSystem,
) -> Result<(), anyhow::Error> {
    let archive = format!(
        "crates-lsp-{arch}-{target}.{ext}",
        target = os.as_target(),
        ext = os.archive_format()
    );

    let uri =
        format!("https://github.com/MathiasPius/crates-lsp/releases/download/v{version}/{archive}");

    crate::info!("downloading {uri}");

    let mut resp = Http::get(&uri)?;
    let body = resp.body_read_all()?;

    // Extract the contained executable.
    match os.archive_format() {
        ArchiveFormat::Zip => {
            let mut archive = ZipArchive::new(Cursor::new(body))?;
            let mut entry = archive.by_index(0)?;
            std::io::copy(&mut entry, &mut file)?;
        }
        ArchiveFormat::TarGz => {
            let mut archive = Archive::new(GzDecoder::new(Cursor::new(body)));
            let mut entry = archive.entries()?.next().unwrap()?;
            std::io::copy(&mut entry, &mut file)?;
        }
    }

    Ok(())
}

pub fn crates_lsp(
    version: SelectedVersion,
    arch: Arch,
    os: OperatingSystem,
) -> Result<Url, anyhow::Error> {
    match version {
        SelectedVersion::Latest => {
            /*
            #[derive(Deserialize)]
            struct GithubRelease {
                tag_name: String,
            }

            let mut resp =
                Http::get("https://api.github.com/repos/MathiasPius/crates-lsp/releases/latest")?;

            let body = resp.body_read_all()?;
            let stringified_body = std::str::from_utf8(&body)?;

            let latest_release: GithubRelease = serde_json::from_str(stringified_body)?;

            crate::info!(
                "latest crates-lsp version: {version}",
                version = latest_release.tag_name
            );

            crates_lsp(
                SelectedVersion::Specific(Version::parse(&latest_release.tag_name[1..])?),
                arch,
                os,
            )
            */

            // I originally did some fancy github api lookup here, but it failed with 403-forbidden,
            // presumeable GitHub refuses to serve content when the user agent is unspecified, since
            // the API endpoint otherwise worked fine when called via cURL and no authentication.
            //
            // Instead I'll just hard code the latest known version here:
            crate::info!("latest crates-lsp version: {LATEST_VERSION}");
            crates_lsp(SelectedVersion::Specific(LATEST_VERSION), arch, os)
        }
        SelectedVersion::Specific(version) => {
            let extension = os.extension();
            let filename = format!("crates-lsp-{version}{extension}");

            if !Path::new(&filename).exists() {
                crate::info!("crates-lsp executable {filename:?} not found, downloading");
                let mut file = std::fs::File::create(&filename)?;
                download_into(&mut file, &version, arch, os)?;
            } else {
                crate::info!("crates-lsp executable {filename:?} already present.");
            }

            let path = Url::parse(&VoltEnvironment::uri()?)?
                .join(&filename)
                .map_err(|_| anyhow::format_err!("unable to parse url from file path"))?;

            crate::info!("crates-lsp version: {version}");
            crates_lsp(SelectedVersion::Url(path), arch, os)
        }
        SelectedVersion::Path(path) => {
            crate::info!("crates-lsp path: {path:?}");
            let url = SelectedVersion::Url(
                Url::parse(&format!("urn:{}", path.to_string_lossy()))
                    .map_err(|_| anyhow::format_err!("unable to parse url from file path"))?,
            );

            crates_lsp(url, arch, os)
        }
        SelectedVersion::Url(url) => {
            crate::info!("crates-lsp url: {url:?}");
            Ok(url)
        }
    }
}
