use crate::conf::Configuration;
use anyhow::Result;
use xcframework::Produced;

pub fn generate(conf: &Configuration, produced: &Produced, resource_dirs: &[&str]) -> Result<()> {
    let module_name = &produced.module_name;
    let package_name = &conf.cargo_section.package_name;

    let xcframework_ext = if produced.is_zipped {
        "xcframework.zip"
    } else {
        "xcframework"
    };
    let resources = resource_dirs
        .iter()
        .map(|dir| format!(".copy(\"{dir}\")"))
        .collect::<Vec<_>>()
        .join(", ");

    let contents = format!(
        r###"// swift-tools-version:5.7.1
import PackageDescription
let package = Package(
  name: "{package_name}",
  products: [
    .library(
      name: "{package_name}",
      targets: ["{package_name}"]),
  ],
  dependencies: [],
  targets: [
    .binaryTarget(
      name: "{module_name}",
      path: "{module_name}.{xcframework_ext}"
    ),
    .target(
      name: "{package_name}",
      dependencies: ["{module_name}"],
      resources: [{resources}]
    )
  ]
)
  "###,
    );
    fs_err::write(conf.build_dir.join("Package.swift"), contents)?;
    Ok(())
}
