mod conf;

use anyhow::Result;
use conf::Configuration;
pub use conf::SpmCli;

pub fn run(cli: SpmCli) -> Result<()> {
    let conf = Configuration::load(cli)?;

    if conf.build_dir.exists() {
        fs_err::remove_dir_all(&conf.build_dir)?;
    }
    fs_err::create_dir_all(&conf.build_dir)?;

    cargo_xcframework::run(conf.cli.to_xc_cli())?;
    Ok(())
}
