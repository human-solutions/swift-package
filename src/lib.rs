mod bindings;
mod conf;
mod swift_package_file;
mod swift_resources_ext;

use anyhow::{anyhow, bail, Context, Result};
use camino_fs::{Utf8Path, Utf8PathExt};
pub use conf::CliArgs;
use conf::Configuration;
use fs_extra::dir::CopyOptions;
use log::LevelFilter;
use simplelog::{
    format_description, ColorChoice, Config as LogConfig, ConfigBuilder, TermLogger, TerminalMode,
};
use xcframework::Produced;

const SWIFT_PACKAGE_UNIFFY_VERSION: &str = env!("UNIFFY_BINDGEN_VERSION");

#[allow(unused_imports)]
#[cfg(not(test))]
use log::{debug, info, warn};

use std::env;
#[allow(unused_imports)]
#[cfg(test)]
use std::{println as info, println as warn, println as debug}; // Workaround to use prinltn! for logs.

pub fn build(cli: CliArgs) -> Result<()> {
    setup_logging(cli.verbose);
    let conf = Configuration::load(cli)?;

    conf.framework_build_dir.rm()?;
    conf.framework_build_dir.mkdirs()?;

    info!("generating bindings...");
    bindings::generate(&conf).context("generating bindings")?;

    if false {
        return Ok(());
    }

    let produced = xcframework::build(&conf.xcframework).context("building with xcframework")?;

    let resource_dirs = copy_resources(&conf)?;
    if !resource_dirs.is_empty() {
        swift_resources_ext::generate(&conf, &resource_dirs)?;
    }
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
            .framework_build_dir
            .join("Sources")
            .join(&conf.cargo_section.package_name);

        copy_dir(dir, &to)?;
        names.push(name);
    }
    Ok(names)
}

fn copy_swift_sources(conf: &Configuration) -> Result<()> {
    let to = conf
        .framework_build_dir
        .join("Sources")
        .join(&conf.cargo_section.package_name);
    to.mkdirs()?;
    let from = &conf.bindings_build_dir;

    from.ls()
        .files()
        .relative_paths()
        .filter(|f| f.extension() == Some("swift"))
        .try_for_each(|f| from.join(&f).cp(to.join(f)))?;
    Ok(())
}

fn move_framework(conf: &Configuration, produced: &Produced) -> Result<()> {
    conf.framework_build_dir.mkdirs()?;

    produced.path.mv(conf
        .framework_build_dir
        .join(produced.path.file_name().unwrap()))?;
    Ok(())
}

pub fn copy_dir(from: &Utf8Path, to: &Utf8Path) -> Result<()> {
    to.mkdirs()?;

    fs_extra::dir::copy(from, to, &CopyOptions::new()).context(format!(
        "Could not recursively copy the directory {from} to {to}"
    ))?;
    Ok(())
}

fn setup_logging(verbose: u32) {
    let log_level = match verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let log_config = if env::var("CARGO").is_ok() && verbose != 0 {
        ConfigBuilder::new()
            .set_time_format_custom(format_description!(
                "cargo::warning=[hour]:[minute]:[second]"
            ))
            .build()
    } else {
        LogConfig::default()
    };

    let _ = TermLogger::init(
        log_level,
        log_config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
}
