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

        let path = PathBuf::from(s);
        if !path.exists() {
            crate::error!("specified crates-lsp version interpreted as a path, but the path does not exist: {}", path.to_string_lossy());
            return Err(anyhow!("specified crates-lsp version interpreted as a path, but the path does not exist: {}", path.to_string_lossy()));
        }

        crate::info!("selected version: {path:?}");
        Ok(SelectedVersion::Path(path))
    }
}
