use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;
use std::process::Command;
use swift_package::SpmCli;
use tempfile::{tempdir, TempDir};
use xcframework::ext::PathBufExt;

#[test]
fn end_to_end_static() {
    let tmp = tempdir().unwrap();
    let target_dir = tmp
        .path()
        .join("mymath-lib/target")
        .to_str()
        .unwrap()
        .to_string();

    let cli = SpmCli::parse_from(&[
        "cargo-xcframework",
        "--quiet",
        "--manifest-path=examples/end-to-end/mymath-lib/Cargo.toml",
        "--target-dir",
        "temp",
    ]);

    swift_package::build(cli).unwrap();

    let swift_dir = cp_swift_exe(&tmp).unwrap();

    let cmd = Command::new("swift")
        .current_dir(&swift_dir)
        .arg("run")
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&cmd.stdout);
    eprintln!("{}", String::from_utf8_lossy(&cmd.stderr));

    assert_eq!("SwiftMath.swift_add(4 + 2) = 6\n", output);
}

fn cp_swift_exe(tmp: &TempDir) -> Result<Utf8PathBuf> {
    let from = Utf8PathBuf::from("examples/end-to-end/swift-exe");
    let to = Utf8PathBuf::from_path_buf(tmp.path().join("swift-exe")).unwrap();

    from.copy_dir_contents(&to)?;
    to.join(".build").remove_dir_all_if_exists()?;
    Ok(to)
}
