mod conf;
mod swift_package_file;
mod swift_resources_ext;

use anyhow::{anyhow, bail, Context, Result};
use camino_fs::{Utf8Path, Utf8PathExt};
pub use conf::CliArgs;
use conf::Configuration;
use fs_extra::dir::CopyOptions;
use xcframework::Produced;

pub fn build(cli: CliArgs) -> Result<()> {
    let conf = Configuration::load(cli)?;

    conf.build_dir.rm()?;
    conf.build_dir.mkdirs()?;

    let produced = xcframework::build(conf.cli.to_xc_cli()).context("building with xcframework")?;

    let resource_dirs = copy_resources(&conf)?;
    swift_resources_ext::generate(&conf, &resource_dirs)?;
    swift_package_file::generate(&conf, &produced, &resource_dirs)
        .context("generate swift package file")?;

    move_framework(&conf, &produced)?;
    copy_swift_sources(&conf)?;

    Ok(())
}
fn copy_resources(conf: &Configuration) -> Result<Vec<&str>> {
    let mut names = vec![];
    for dir in &conf.cargo_section.resource_dirs {
        if !dir.is_dir() {
            bail!("Expected a resource dir: {dir} but that's not a directory");
        }

        let name = dir.iter().last().ok_or(anyhow!("Empty dir: {dir}"))?;
        let to = conf
            .build_dir
            .join("Sources")
            .join(&conf.cargo_section.package_name);

        copy_dir(dir, &to)?;
        names.push(name);
    }
    Ok(names)
}

fn copy_swift_sources(conf: &Configuration) -> Result<()> {
    let from = &conf.cargo_section.swift_source_dir;
    let to = conf
        .build_dir
        .join("Sources")
        .join(&conf.cargo_section.package_name);

    copy_dir(from, &to)
}

fn move_framework(conf: &Configuration, produced: &Produced) -> Result<()> {
    conf.build_dir.mkdirs()?;

    produced
        .path
        .mv(conf.build_dir.join(produced.path.file_name().unwrap()))?;
    Ok(())
}

pub fn copy_dir(from: &Utf8Path, to: &Utf8Path) -> Result<()> {
    to.mkdirs()?;

    fs_extra::dir::copy(from, to, &CopyOptions::new()).context(format!(
        "Could not recursively copy the directory {from} to {to}"
    ))?;
    Ok(())
}
