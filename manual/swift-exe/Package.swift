// swift-tools-version:5.7.1
import PackageDescription

let package = Package(
    name: "GreeterTest",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_15),
    ],
    products: [],
    dependencies: [.package(path: "../greeter/target/Greeter")],
    targets: [
        .executableTarget(
            name: "swift_cmd",
            dependencies: [.product(name: "Greeter", package: "Greeter")])
    ]
)
