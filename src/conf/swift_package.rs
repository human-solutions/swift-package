use anyhow::{bail, Context, Result};
use camino_fs::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SwiftPackageConfiguration {
    pub package_name: String,
    pub udl_file: Utf8PathBuf,
    pub swift_source_dir: Utf8PathBuf,
    #[serde(default)]
    pub resource_dirs: Vec<Utf8PathBuf>,
}

impl SwiftPackageConfiguration {
    /// Parses the [package.metadata.swift-package] section of the Cargo.toml
    /// and updates the headers_directory to be relative to current working directory
    pub fn parse(metadata: &serde_json::Value, package_dir: &Utf8Path) -> Result<Self> {
        if let Some(xcfr) = metadata.get("swift-package") {
            Self::parse_swift_package(xcfr, package_dir)
                .context("Error in Cargo.toml section [package.metadata.swift-package]")
        } else {
            bail!("Missing [package.metadata.swift-package] section in Cargo.toml");
        }
    }

    fn parse_swift_package(xcfr: &serde_json::Value, package_dir: &Utf8Path) -> Result<Self> {
        let mut me = serde_json::from_value::<Self>(xcfr.clone())?;
        me.swift_source_dir = package_dir.join(me.swift_source_dir);
        me.resource_dirs = me
            .resource_dirs
            .into_iter()
            .map(|res| package_dir.join(res))
            .collect();
        me.udl_file = package_dir.join(me.udl_file);
        Ok(me)
    }
}
