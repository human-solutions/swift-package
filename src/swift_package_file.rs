use crate::conf::Configuration;
use anyhow::Result;
use cargo_xcframework::Produced;

pub fn generate(conf: &Configuration, produced: &Produced) -> Result<()> {
    let module_name = &produced.module_name;
    let xcframework_ext = if produced.is_zipped {
        "xcframework.zip"
    } else {
        "xcframework"
    };

    let contents = format!(
        r###"// swift-tools-version:5.7.1
import PackageDescription
let package = Package(
  name: "SwiftMath",
  products: [
    .library(
      name: "SwiftMath",
      targets: ["SwiftMath"]),
  ],
  dependencies: [],
  targets: [
    .binaryTarget(
      name: "{module_name}",
      path: "{module_name}.{xcframework_ext}"
    ),
    .target(
      name: "SwiftMath",
      dependencies: ["{module_name}"]
    )
  ]
)
  "###,
    );
    fs_err::write(conf.build_dir.join("Package.swift"), contents)?;
    Ok(())
}
