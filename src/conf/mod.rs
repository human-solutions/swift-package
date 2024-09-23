mod args;
mod configuration;
mod swift_package;

pub use args::SwiftPackage as CliArgs;
pub use configuration::Configuration;
pub use swift_package::SwiftPackageConfiguration;
use xcframework::CliArgs as XcCli;

impl CliArgs {
    pub fn to_xc_cli(&self) -> XcCli {
        XcCli {
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
