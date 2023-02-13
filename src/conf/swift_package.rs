use anyhow::{bail, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SwiftPackageConfiguration {
    pub swift_source_dir: Utf8PathBuf,
}

impl SwiftPackageConfiguration {
    /// Parses the [package.metadata.swift-package] section of the Cargo.toml
    /// and updates the headers_directory to be relative to current working directory
    pub fn parse(metadata: &serde_json::Value, dir: &Utf8Path) -> Result<Self> {
        if let Some(xcfr) = metadata.get("swift-package") {
            Self::parse_xcframework(xcfr, dir)
                .context("Error in Cargo.toml section [package.metadata.swift-package]")
        } else {
            bail!("Missing [package.metadata.swift-package] section in Cargo.toml");
        }
    }

    fn parse_xcframework(xcfr: &serde_json::Value, _dir: &Utf8Path) -> Result<Self> {
        Ok(serde_json::from_value::<Self>(xcfr.clone())?)
    }
}
