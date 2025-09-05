// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "LocalNotification",
    platforms: [
        .macOS(.v10_15),
        .iOS(.v13),
        .visionOS(.v1),
    ],
    products: [
        .library(
            name: "LocalNotification",
            type: .static,
            targets: ["LocalNotification"])
    ],
    dependencies: [
        .package(url: "https://github.com/Brendonovich/swift-rs", from: "1.0.7")
    ],
    targets: [
        .target(
            name: "LocalNotification",
            dependencies: [
                .product(
                    name: "SwiftRs",
                    package: "swift-rs"
                )
            ],
        )
    ]
)
