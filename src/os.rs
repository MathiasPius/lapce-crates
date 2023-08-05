use anyhow::anyhow;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

impl FromStr for OperatingSystem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "macos" => Ok(OperatingSystem::MacOS),
            "linux" => Ok(OperatingSystem::Linux),
            "windows" => Ok(OperatingSystem::Windows),
            os => Err(anyhow!("Unsupported operating system: {os}")),
        }
    }
}

impl OperatingSystem {
    pub fn as_target(&self) -> &'static str {
        match self {
            OperatingSystem::Windows => "pc-windows-msvc",
            OperatingSystem::MacOS => "apple-darwin",
            OperatingSystem::Linux => "unknown-linux-gnu",
        }
    }

    pub fn archive_format(&self) -> ArchiveFormat {
        match self {
            OperatingSystem::Windows => ArchiveFormat::Zip,
            OperatingSystem::MacOS => ArchiveFormat::TarGz,
            OperatingSystem::Linux => ArchiveFormat::TarGz,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            OperatingSystem::Windows => ".exe",
            OperatingSystem::MacOS | OperatingSystem::Linux => "",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ArchiveFormat {
    Zip,
    TarGz,
}

impl Display for ArchiveFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ArchiveFormat::Zip => "zip",
            ArchiveFormat::TarGz => "tar.gz",
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Arch {
    X86_64,
    Aarch64,
}

impl FromStr for Arch {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x86_64" => Ok(Arch::X86_64),
            "aarch64" => Ok(Arch::Aarch64),
            arch => Err(anyhow!("Unsupported architecture: {arch}")),
        }
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Arch::X86_64 => "x86_64",
            Arch::Aarch64 => "aarch64",
        })
    }
}
