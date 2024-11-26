use crate::SWIFT_PACKAGE_UNIFFY_VERSION;

use super::{CliArgs, SwiftPackageConfiguration};
use anyhow::{anyhow, bail, Result};
use camino_fs::Utf8PathBuf;
use cargo_metadata::{Metadata, MetadataCommand, Package};

#[derive(Debug)]
pub struct Configuration {
    pub target_name: String,
    pub cargo_section: SwiftPackageConfiguration,
    pub cli: CliArgs,

    /// When in a workspace configuration, this is different
    /// from the package dir.
    pub manifest_dir: Utf8PathBuf,
    pub target_dir: Utf8PathBuf,
    pub framework_build_dir: Utf8PathBuf,
    pub bindings_build_dir: Utf8PathBuf,
}

impl Configuration {
    pub fn load(cli: CliArgs) -> Result<Self> {
        let manifest_path = cli
            .manifest_path
            .clone()
            .unwrap_or_else(|| Utf8PathBuf::from("Cargo.toml"));
        let mut manifest_dir = manifest_path.clone();
        manifest_dir.pop();

        let target_dir = cli
            .target_dir
            .clone()
            .unwrap_or_else(|| manifest_dir.join("target"));

        let metadata = MetadataCommand::new().manifest_path(manifest_path).exec()?;

        let package = if let Some(package) = &cli.package {
            metadata
                .workspace_packages()
                .iter()
                .find(|p| &p.name == package)
                .ok_or(anyhow!("Could not find package '{package}'"))?
        } else {
            metadata
                .root_package()
                .ok_or(anyhow!("Could not find root package in metadata"))?
        };
        let package_dir = package.manifest_path.parent().unwrap();

        uniffi_version_check(&package, &metadata)?;
        let target_name = find_target_name(&package)?;

        let sp_conf = SwiftPackageConfiguration::parse(&package.metadata, &package_dir)?;

        let build_dir = target_dir.join(format!("lib{target_name}"));
        let framework_build_dir = build_dir.join(format!("{target_name}.package"));
        Ok(Self {
            cargo_section: sp_conf,
            cli,
            manifest_dir,
            framework_build_dir,
            bindings_build_dir: build_dir.join("bindings"),
            target_name,
            target_dir,
        })
    }

    pub fn dylib_file(&self) -> Utf8PathBuf {
        let profile = if self.cli.release { "release" } else { "debug" };
        self.target_dir
            .join(profile)
            .join(format!("lib{}.dylib", self.target_name))
    }
}

fn find_target_name(package: &Package) -> Result<String> {
    let target = package
        .targets
        .iter()
        .find(|t| t.kind.iter().any(|k| k == "cdylib"))
        .ok_or(anyhow!(
            "Could not find a cdylib target in package {}",
            package.name
        ))?;
    Ok(target.name.clone())
}

fn uniffi_version_check(package: &Package, metadata: &Metadata) -> Result<()> {
    package
        .dependencies
        .iter()
        .find(|dep| dep.name == "uniffi")
        .ok_or(anyhow!(
            "The package {} should have a uniffi dependency",
            package.name
        ))?;
    let uniffi_bindgen_version = metadata
        .packages
        .iter()
        .find(|pack| pack.name == "uniffi")
        .unwrap()
        .version
        .to_string();

    let expected = major_and_minor(SWIFT_PACKAGE_UNIFFY_VERSION);
    let found = major_and_minor(&uniffi_bindgen_version);
    if expected != found {
        bail!(
            "uniffi_bindgen version mismatch: \
            swift-package is build with {SWIFT_PACKAGE_UNIFFY_VERSION} \
            but the project uses {uniffi_bindgen_version}"
        );
    }
    Ok(())
}

fn major_and_minor(semver: &str) -> String {
    let mut parts = semver.split('.');
    let major = parts.next().unwrap();
    let minor = parts.next().unwrap();
    format!("{}.{}", major, minor)
}
