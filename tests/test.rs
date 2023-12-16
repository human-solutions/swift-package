use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;
use fs_err as fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use swift_package::SpmCli;

fn create_output_dir(subfolder: &str) -> PathBuf {
    let tmp_dir = PathBuf::from("tests").join("temp").join(subfolder);
    if tmp_dir.exists() {
        fs::remove_dir_all(&tmp_dir).unwrap();
    }
    fs::create_dir_all(&tmp_dir).unwrap();
    tmp_dir
}

#[test]
fn end_to_end_static() {
    let out_dir = create_output_dir("static");

    let target_dir = out_dir.join("mymath-lib/target");
    fs::create_dir_all(&target_dir).unwrap();

    let cli = SpmCli::parse_from([
        "cargo-xcframework",
        "--quiet",
        "--manifest-path=examples/end-to-end/mymath-lib/Cargo.toml",
        "--target-dir",
        &target_dir.to_str().unwrap(),
    ]);

    swift_package::build(cli).unwrap();

    let swift_dir = cp_swift_exe(&out_dir).unwrap();

    let cmd = Command::new("swift")
        .current_dir(swift_dir)
        .arg("run")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&cmd.stdout);
    let stderr = String::from_utf8_lossy(&cmd.stderr);
    eprintln!("{stderr}");
    assert!(stderr.contains("Build complete!"));
    assert_eq!(
        "SwiftMath.swift_add(4 + 2) = 6; from resource file: hi there\n",
        stdout
    );
}

fn cp_swift_exe(dest: &Path) -> Result<Utf8PathBuf> {
    let from = Utf8PathBuf::from("examples/end-to-end/swift-exe");

    let dest = Utf8PathBuf::from_path_buf(dest.to_path_buf()).unwrap();
    fs::create_dir_all(&dest)?;

    fs_extra::dir::copy(from, &dest, &fs_extra::dir::CopyOptions::new())?;
    let build_tmp = dest.join("swift-exe/.build");
    if build_tmp.exists() {
        fs::remove_dir_all(build_tmp)?;
    }
    Ok(dest.join("swift-exe"))
}
