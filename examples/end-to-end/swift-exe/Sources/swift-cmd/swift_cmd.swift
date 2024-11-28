import Foundation
import SwiftMath

@main
public struct swift_cmd {

    public static func main() {
        let helloFile = SwiftMath.resources(name: "hello.txt")
        let hello = try! String(contentsOf: helloFile)

        let sum = SwiftMath.rustAdd(a: 4, b: 2)
        print(
            "SwiftMath.swift_add(4 + 2) = \(sum); from resource file: \(hello)"
        )
    }
}
