use std::process::Command;

use crate::conf::Configuration;
use anyhow::{Context, Result};
use camino_fs::{Utf8Path, Utf8PathBuf, Utf8PathExt};
use uniffi_bindgen::{bindings::SwiftBindingGenerator, generate_external_bindings};

pub fn generate(conf: &Configuration) -> Result<()> {
    build_lib(conf).context("building lib before generating bindings")?;
    println!(
        "Generating Swift bindings for {}",
        conf.cargo_section.package_name
    );
    generate_external_bindings(
        &SwiftBindingGenerator,
        &conf.cargo_section.udl_file,
        None::<&Utf8PathBuf>,
        Some(&conf.bindings_build_dir),
        Some(conf.dylib_file()),
        Some(&conf.target_name),
        false,
    )?;
    fix_modulemap_file(&conf.bindings_build_dir);
    Ok(())
}

fn build_lib(conf: &Configuration) -> Result<()> {
    let manif_arg = format!("--manifest-path={}/Cargo.toml", conf.manifest_dir);

    let mut args = vec!["build", "--color=always", &manif_arg, "--lib"];
    if conf.cli.release {
        args.push("--release");
    }

    let package = conf.cli.package.as_ref().map(|p| format!("--package={p}"));
    if let Some(package) = &package {
        args.push(&package);
    }

    let target_dir = conf
        .cli
        .target_dir
        .as_ref()
        .map(|d| format!("--target-dir={d}"));
    if let Some(target_dir) = &target_dir {
        args.push(&target_dir);
    }

    let out = Command::new("cargo").args(args).output()?;

    if !out.status.success() {
        panic!(
            "Failed to build the library: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(())
}

/// the generated module file starts with "module " but it should be "framework module "
fn fix_modulemap_file(out_dir: &Utf8Path) {
    let modulemap_file = out_dir
        .ls()
        .files()
        .filter(|f| f.extension() == Some("modulemap"))
        .next()
        .unwrap();

    let modulemap = modulemap_file.read_string().unwrap();

    if !modulemap.starts_with("module ") {
        panic!("modulemap file does not start with 'module '")
    }

    let mut new_modulemap = String::with_capacity(modulemap.len() + 10);
    new_modulemap.push_str("framework ");
    new_modulemap.push_str(&modulemap);

    modulemap_file.write(new_modulemap.as_bytes()).unwrap();
}
