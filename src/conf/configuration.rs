use crate::SWIFT_PACKAGE_UNIFFY_VERSION;

use super::{CliArgs, SwiftPackageConfiguration};
use anyhow::{anyhow, bail, Context, Result};
use camino_fs::Utf8PathBuf;
use cargo_metadata::{Metadata, MetadataCommand, Package, TargetKind};
use xcframework::{Configuration as XCMainConfig, XCFrameworkConfiguration};

#[derive(Debug)]
pub struct Configuration {
    pub target_name: String,
    pub cargo_section: SwiftPackageConfiguration,
    pub xcframework: XCMainConfig,
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

        let metadata = MetadataCommand::new().manifest_path(manifest_path).exec()?;

        let target_dir = cli
            .target_dir
            .clone()
            .unwrap_or_else(|| metadata.target_directory.clone());

        let workspace_packages = metadata.workspace_packages();
        let package = if let Some(package) = &cli.package {
            workspace_packages
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
        let Some(section) = package.metadata.get("swift-package") else {
            bail!("Missing '[package.metadata.swift-package]' section in Cargo.toml")
        };
        let sp_conf = SwiftPackageConfiguration::parse(&section, &package_dir).context(
            "Error when creating swift package configuration by parsing \
                Cargo.toml section [package.metadata.swift-package]",
        )?;
        let target_name = find_target_name(&package)?;
        let build_dir = target_dir.join(format!("lib{target_name}"));
        let framework_build_dir = build_dir.join(format!("{}.package", sp_conf.package_name));
        let bindings_build_dir = build_dir.join("bindings");

        let mut xc_conf = XCFrameworkConfiguration::parse(&section, &package_dir, false).context(
            "Error when creating xcframework configuration by parsing \
                Cargo.toml section [package.metadata.swift-package]",
        )?;
        xc_conf.include_dir = bindings_build_dir.clone();
        let xc_cli = cli.to_xc_cli();
        let xcframework = XCMainConfig::new(&metadata, package, xc_cli, xc_conf)?;

        Ok(Self {
            cargo_section: sp_conf,
            xcframework,
            cli,
            manifest_dir,
            framework_build_dir,
            bindings_build_dir,
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
        .find(|t| t.kind.iter().any(|k| *k == TargetKind::CDyLib))
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
