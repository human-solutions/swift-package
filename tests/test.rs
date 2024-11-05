use anyhow::Result;
use camino_fs::{Utf8Path, Utf8PathBuf, Utf8PathExt};
use std::process::Command;

use swift_package::CliArgs;

fn create_output_dir(subfolder: &str) -> Utf8PathBuf {
    let tmp_dir = Utf8PathBuf::from("tests").join("temp").join(subfolder);
    tmp_dir.rm().unwrap();
    tmp_dir.mkdirs().unwrap();
    tmp_dir
}

fn args(vec: &[&str]) -> CliArgs {
    CliArgs::from_vec(vec.iter().map(|s| s.into()).collect()).unwrap()
}

#[test]
fn end_to_end_static() {
    let out_dir = create_output_dir("static");

    let target_dir = out_dir.join("mymath-lib/target");
    target_dir.mkdirs().unwrap();

    let cli = args(&[
        "--quiet",
        "--manifest-path",
        "examples/end-to-end/mymath-lib/Cargo.toml",
        "--target-dir",
        &target_dir.as_str(),
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
    assert!(stderr.contains("complete!"));
    assert_eq!(
        "SwiftMath.swift_add(4 + 2) = 6; from resource file: hi there\n",
        stdout
    );
}

fn cp_swift_exe(dest: &Utf8Path) -> Result<Utf8PathBuf> {
    let from = Utf8PathBuf::from("examples/end-to-end/swift-exe");

    let dest = dest.join("swift-exe");
    dest.mkdirs()?;

    from.cp(&dest)?;
    let build_tmp = dest.join(".build");
    build_tmp.rm()?;
    Ok(dest)
}
