use std::{path::PathBuf, str::FromStr};

use lapce_plugin::psp_types::lsp_types::Url;
use semver::Version;

use anyhow::anyhow;

pub enum SelectedVersion {
    Latest,
    Specific(Version),
    Path(PathBuf),
    Url(Url),
}

impl FromStr for SelectedVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.to_lowercase() == "latest" || s.to_lowercase() == "*" {
            crate::info!("selected version: latest");
            return Ok(SelectedVersion::Latest);
        }

        if let Ok(version) = Version::parse(s) {
            crate::info!("selected version: {version}");
            return Ok(SelectedVersion::Specific(version));
        }

        if let Ok(path) = PathBuf::try_from(s) {
            crate::info!("selected version: {path:?}");
            return Ok(SelectedVersion::Path(path));
        }

        crate::error!("specified crates-lsp version is not valid: {s}");
        Err(anyhow!("Specified version is not a valid version"))
    }
}
