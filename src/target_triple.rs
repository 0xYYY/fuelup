use anyhow::{bail, Result};
use component::{self, Component};
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct TargetTriple(String);

impl fmt::Display for TargetTriple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TargetTriple {
    pub fn new(s: &str) -> Result<Self> {
        let (architecture, rest) = match s.split_once('-') {
            Some((architecture, rest)) => (architecture, rest),
            None => bail!("missing vendor-os specifier"),
        };

        if !["aarch64", "x86_64"].contains(&architecture) {
            bail!("Unsupported architecture: '{}'", architecture);
        }

        let (vendor, os) = match rest.split_once('-') {
            Some((vendor, os)) => (vendor, os),
            None => bail!("missing os specifier"),
        };

        if !["apple", "unknown"].contains(&vendor) {
            bail!("Unsupported vendor: '{}'", vendor);
        }

        if !["darwin", "linux-gnu"].contains(&os) {
            bail!("Unsupported os: '{}'", os);
        }

        Ok(Self(s.to_string()))
    }

    pub fn from_host() -> Result<Self> {
        let architecture = match std::env::consts::ARCH {
            "aarch64" | "x86_64" => std::env::consts::ARCH,
            unsupported_arch => bail!("Unsupported architecture: {}", unsupported_arch),
        };
        let vendor = match std::env::consts::OS {
            "macos" => "apple",
            _ => "unknown",
        };
        let os = match std::env::consts::OS {
            "macos" => "darwin",
            "linux" => "linux-gnu",
            unsupported_os => bail!("Unsupported os: {}", unsupported_os),
        };

        let target_triple = format!("{}-{}-{}", architecture, vendor, os);

        Ok(Self(target_triple))
    }

    pub fn from_component(component: &str) -> Result<Self> {
        match Component::from_name(component).map(|c| c.name)?.as_str() {
            component::FORC => {
                let os = match std::env::consts::OS {
                    "macos" => "darwin",
                    "linux" => "linux",
                    unsupported_os => bail!("Unsupported os: {}", unsupported_os),
                };
                let architecture = match std::env::consts::ARCH {
                    "aarch64" => "arm64",
                    "x86_64" => "amd64",
                    unsupported_arch => bail!("Unsupported architecture: {}", unsupported_arch),
                };

                Ok(Self(format!("{}_{}", os, architecture)))
            }
            _ => {
                let architecture = match std::env::consts::ARCH {
                    "aarch64" | "x86_64" => std::env::consts::ARCH,
                    unsupported_arch => bail!("Unsupported architecture: {}", unsupported_arch),
                };

                let vendor = match std::env::consts::OS {
                    "macos" => "apple",
                    _ => "unknown",
                };

                let os = match std::env::consts::OS {
                    "macos" => "darwin",
                    "linux" => "linux-gnu",
                    unsupported_os => bail!("Unsupported os: {}", unsupported_os),
                };

                Ok(Self(format!("{}-{}-{}", architecture, vendor, os)))
            }
        }
    }
}
