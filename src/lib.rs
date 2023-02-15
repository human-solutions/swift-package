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

    if conf.build_dir.exists() {
        fs_err::remove_dir_all(&conf.build_dir)?;
    }
    fs_err::create_dir_all(&conf.build_dir)?;
    let produced = cargo_xcframework::run(conf.cli.to_xc_cli())?;

    swift_package_file::generate(&conf, &produced)?;
    fs_err::create_dir_all(conf.build_dir.join("Sources").join("SwiftMath"))?;

    copy_framework(&conf, &produced)?;
    copy_swift_file(&conf)?;
    fs_err::copy(
        conf.dir.join("swift").join("SwiftMath.swift"),
        conf.build_dir.join("SwiftMath.swift"),
    )?;

    Ok(())
}

fn copy_swift_file(conf: &Configuration) -> Result<()> {
    let from = &conf.cargo_section.swift_source_dir;
    let to = conf.build_dir.join("Sources").join("SwiftMath");

    if !to.exists() {
        fs_err::create_dir(&to)?;
    }
    let options = CopyOptions::new();
    fs_extra::dir::copy(&from, &to, &options).context(format!(
        "Could not recursively copy the directory {from} to {to}"
    ))?;
    Ok(())
}
fn copy_framework(conf: &Configuration, produced: &Produced) -> Result<()> {
    conf.build_dir.create_dir_all_if_needed()?;

    if produced.is_zipped {
        fs_err::copy(
            &produced.path,
            &conf.build_dir.join(produced.path.file_name().unwrap()),
        )?;
    } else {
        let options = CopyOptions::new();
        fs_extra::dir::copy(&produced.path, &conf.build_dir, &options).context(format!(
            "Could not recursively copy the directory {} to {}",
            &produced.path, &conf.build_dir
        ))?;
    }
    Ok(())
}
