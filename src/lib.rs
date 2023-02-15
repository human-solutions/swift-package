mod conf;
mod ext;
mod swift_package_file;

use anyhow::{Context, Result};
use cargo_xcframework::Produced;
use conf::Configuration;
pub use conf::SpmCli;
use ext::PathBufExt;
use fs_extra::dir::CopyOptions;

pub fn run(cli: SpmCli) -> Result<()> {
    let conf = Configuration::load(cli)?;

    conf.build_dir.remove_dir_all_if_exists()?;
    fs_err::create_dir_all(&conf.build_dir)?;

    let produced = cargo_xcframework::build(conf.cli.to_xc_cli())?;

    swift_package_file::generate(&conf, &produced)?;

    move_framework(&conf, &produced)?;
    copy_swift_file(&conf)?;

    Ok(())
}

fn copy_swift_file(conf: &Configuration) -> Result<()> {
    let from = &conf.cargo_section.swift_source_dir;
    let to = conf.build_dir.join("Sources").join("SwiftMath");
    to.create_dir_all_if_needed()?;

    let options = CopyOptions::new();
    fs_extra::dir::copy(&from, &to, &options).context(format!(
        "Could not recursively copy the directory {from} to {to}"
    ))?;
    Ok(())
}
fn move_framework(conf: &Configuration, produced: &Produced) -> Result<()> {
    conf.build_dir.create_dir_all_if_needed()?;

    fs_err::rename(
        &produced.path,
        &conf.build_dir.join(produced.path.file_name().unwrap()),
    )?;
    Ok(())
}
