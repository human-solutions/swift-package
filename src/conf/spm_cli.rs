use camino::Utf8PathBuf;
use clap::Parser;
use xcframework::CliArgs;

/// Compile a package into a cross-platform Apple XCFramework
#[derive(Debug, Parser)]
#[clap(version)]
pub struct SpmCli {
    /// Do not print cargo log messages
    #[arg(short, long)]
    pub quiet: bool,

    /// Package to build (see `cargo help pkgid`)
    #[arg(short, long)]
    pub package: Option<String>,

    /// Use verbose output (-vv very verbose/build.rs output)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    #[arg(short = 'Z', value_name = "FLAG")]
    pub unstable_flags: Option<String>,

    /// Build artifacts in release mode, with optimizations
    #[arg(short, long)]
    pub release: bool,

    /// Build artifacts with the specified profile
    #[arg(long, value_name = "PROFILE-NAME")]
    pub profile: Option<String>,

    /// Space or comma separated list of features to activate
    #[arg(short, long)]
    pub features: Vec<String>,

    /// Activate all available features
    #[arg(long)]
    pub all_features: bool,

    /// Do not activate the `default` feature
    #[arg(long)]
    pub no_default_features: bool,

    /// Directory for all generated artifacts
    #[arg(long, value_name = "DIRECTORY")]
    pub target_dir: Option<Utf8PathBuf>,

    /// Path to Cargo.toml.
    #[arg(long, value_name = "PATH")]
    pub manifest_path: Option<Utf8PathBuf>,
}

impl SpmCli {
    pub fn to_xc_cli(&self) -> CliArgs {
        CliArgs {
            lib_type: None,
            quiet: self.quiet,
            package: self.package.clone(),
            verbose: self.verbose as u32,
            unstable_flags: self.unstable_flags.clone(),
            release: self.release,
            profile: self.profile.clone(),
            features: self.features.clone(),
            all_features: self.all_features,
            no_default_features: self.no_default_features,
            target_dir: self.target_dir.clone(),
            manifest_path: self.manifest_path.clone(),
        }
    }
}
