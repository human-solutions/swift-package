# Swift test of XCFramework

## Build the package

First, build the XCFramework:

```bash
# In the root of the repo:
cargo run -- --manifest-path examples/end-to-end/mymath-lib/Cargo.toml
```

Then, run the Swift executable:

```bash
# In the swift-exe directory:
swift run
```
