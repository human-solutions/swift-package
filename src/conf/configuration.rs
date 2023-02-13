use super::SwiftPackageConfiguration;
use crate::SpmCli;
use anyhow::Result;
use camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;
use cargo_xcframework::XCFrameworkConfiguration;

#[derive(Debug)]
pub struct Configuration {
    /// The root dir of the project
    pub dir: Utf8PathBuf,
    pub cargo_section: SwiftPackageConfiguration,
    pub cli: SpmCli,
    pub xcframework: XCFrameworkConfiguration,
    /// Directory for all generated artifacts
    pub target_dir: Utf8PathBuf,
    /// Directory where the xcframework will be built
    pub build_dir: Utf8PathBuf,
}

impl Configuration {
    pub fn load(cli: SpmCli) -> Result<Self> {
        let manifest_path = cli
            .manifest_path
            .clone()
            .unwrap_or_else(|| Utf8PathBuf::from("Cargo.toml"));
        let mut dir = manifest_path.clone();
        dir.pop();

        let target_dir = dir.join(
            cli.target_dir
                .clone()
                .unwrap_or_else(|| Utf8PathBuf::from("target")),
        );
        let build_dir = target_dir.join("xcframework");

        let metadata = MetadataCommand::new().manifest_path(manifest_path).exec()?;

        let Some(package) = metadata.root_package() else {
            anyhow::bail!("Could not find root package in metadata");
        };
        let xc_conf = XCFrameworkConfiguration::parse(&package.metadata, &dir)?;
        let sp_conf = SwiftPackageConfiguration::parse(&package.metadata, &dir)?;

        Ok(Self {
            dir,
            cargo_section: sp_conf,
            xcframework: xc_conf,
            cli,
            target_dir,
            build_dir,
        })
    }
}
