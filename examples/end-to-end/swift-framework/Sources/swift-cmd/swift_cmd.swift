import SwiftMath
import Foundation

@main
public struct swift_cmd {

    public static func main() {
        let helloFile = SwiftMath.resources.appendingPathComponent("hello.txt")
        let hello = try! String(contentsOf: helloFile)
        print("SwiftMath.swift_add(4 + 2) = \(SwiftMath.swift_add(4, 2)); from resource file: \(hello)")
    }
}
