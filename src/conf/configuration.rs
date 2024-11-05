use super::{CliArgs, SwiftPackageConfiguration};
use anyhow::{anyhow, Result};
use camino_fs::Utf8PathBuf;
use cargo_metadata::MetadataCommand;

#[derive(Debug)]
pub struct Configuration {
    pub cargo_section: SwiftPackageConfiguration,
    pub cli: CliArgs,
    pub build_dir: Utf8PathBuf,
}

impl Configuration {
    pub fn load(cli: CliArgs) -> Result<Self> {
        let manifest_path = cli
            .manifest_path
            .clone()
            .unwrap_or_else(|| Utf8PathBuf::from("Cargo.toml"));
        let mut dir = manifest_path.clone();
        dir.pop();

        let target_dir = cli.target_dir.clone().unwrap_or_else(|| dir.join("target"));

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

        let sp_conf = SwiftPackageConfiguration::parse(&package.metadata, &dir)?;

        let build_dir = target_dir.join(format!("{}.package", sp_conf.package_name));
        Ok(Self {
            cargo_section: sp_conf,
            cli,
            build_dir,
        })
    }
}
