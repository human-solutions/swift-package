// swift-tools-version:5.7.1
import PackageDescription

let package = Package(
    name: "mymath",
    products: [],
    dependencies: [.package(path: "../mymath-lib/target/libmymath/SwiftMath.package")],
    targets: [
        .executableTarget(
            name: "swift-cmd",
            dependencies: [.product(name: "SwiftMath", package: "SwiftMath.package")])
    ]
)
