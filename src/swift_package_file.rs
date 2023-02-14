use crate::conf::Configuration;
use anyhow::Result;

pub fn generate(conf: &Configuration) -> Result<()> {
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
      name: "mymath",
      path: "mymath.xcframework"
    ),
    .target(
      name: "SwiftMath",
      dependencies: ["mymath"]
    )
  ]
)
  "###,
    );
    fs_err::write(conf.build_dir.join("Package.swift"), contents)?;
    Ok(())
}
