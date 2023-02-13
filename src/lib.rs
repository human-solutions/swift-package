mod conf;

use anyhow::Result;
pub use conf::Cli;
use conf::Configuration;

pub fn run(cli: Cli) -> Result<()> {
    let conf = Configuration::load(cli)?;

    if conf.build_dir.exists() {
        fs_err::remove_dir_all(&conf.build_dir)?;
    }
    fs_err::create_dir_all(&conf.build_dir)?;

    Ok(())
}
