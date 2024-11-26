// swift-tools-version:5.7.1
import PackageDescription

let package = Package(
    name: "mymath",
    products: [],
    targets: [
        .executableTarget(
            name: "swift-cmd",
            dependencies: [.product(name: "SwiftMath", package: "SwiftMath.package")]
        ),
        .binaryTarget(
            name: "SwiftMath",
            path: "../mymath-lib/target/SwiftMath.package/mymathFFI.xcframework"
        ),
    ]
)
