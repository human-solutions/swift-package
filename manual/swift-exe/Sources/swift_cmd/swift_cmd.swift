import Foundation
import Greeter

@main
public struct swift_cmd {

    public static func main() {
        print("Greeting: \(Greeter.greet(name: "uniffi"))")
    }
}
