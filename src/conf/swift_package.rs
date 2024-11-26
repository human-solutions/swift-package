use anyhow::Result;
use camino_fs::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SwiftPackageConfiguration {
    pub package_name: String,
    pub udl_file: Utf8PathBuf,
    #[serde(default)]
    pub resource_dirs: Vec<Utf8PathBuf>,
}

impl SwiftPackageConfiguration {
    /// Parses the [package.metadata.swift-package] section of the Cargo.toml
    pub fn parse(xcfr: &serde_json::Value, package_dir: &Utf8Path) -> Result<Self> {
        let mut me = serde_json::from_value::<Self>(xcfr.clone())?;
        me.resource_dirs = me
            .resource_dirs
            .into_iter()
            .map(|res| package_dir.join(res))
            .collect();
        me.udl_file = package_dir.join(me.udl_file);
        Ok(me)
    }
}
