// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: Greeter

import PackageDescription

let package = Package(
    name: "Greeter",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_15),
    ],
    products: [
        .library(
            name: "Greeter",
            targets: ["Greeter"]
        )
    ],
    dependencies: [],
    targets: [
        .binaryTarget(name: "RustFramework", path: "./RustFramework.xcframework"),
        .target(
            name: "Greeter",
            dependencies: [
                .target(name: "RustFramework")
            ]
        ),
    ]
)
