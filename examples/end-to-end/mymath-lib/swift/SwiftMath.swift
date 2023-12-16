import MyMath

public struct SwiftMath {
    public static func swift_add(_ a: Int32, _ b: Int32) -> Int32 {
        MyMath.rust_add(a, b)
    }
}