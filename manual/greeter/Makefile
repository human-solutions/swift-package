STATIC_LIB_NAME := libgreeter.a

apple:
	@make build-targets
	@make universal-binaries
	@make bindgen-swift
	@make xcframework
	@make swift-framework

build-targets:
	cargo build --lib --target aarch64-apple-darwin --target x86_64-apple-darwin --target x86_64-apple-ios --target aarch64-apple-ios-sim --target aarch64-apple-ios

bindgen-swift:
	cargo build --lib
	cargo run --bin uniffi-bindgen --features=uniffi/cli -- \
	           generate --language swift --no-format --lib-file target/debug/libgreeter.dylib src/greeter.udl --out-dir target/bindings
	mkdir -p target/bindings/headers && mv target/bindings/*.h target/bindings/headers && mv target/bindings/greeterFFI.modulemap target/bindings/headers/module.modulemap
	mkdir -p target/bindings/sources && mv target/bindings/*.swift target/bindings/sources

universal-binaries:
	mkdir -p target/universal-ios/debug && mkdir -p target/universal-mac/debug
	lipo -create target/x86_64-apple-ios/debug/$(STATIC_LIB_NAME) target/aarch64-apple-ios-sim/debug/$(STATIC_LIB_NAME) -output target/universal-ios/debug/$(STATIC_LIB_NAME)
	lipo -create target/aarch64-apple-darwin/debug/$(STATIC_LIB_NAME) target/x86_64-apple-darwin/debug/$(STATIC_LIB_NAME) -output target/universal-mac/debug/$(STATIC_LIB_NAME)

xcframework:
	rm -rf target/RustFramework.xcframework || echo "skip removing"
	xcodebuild -create-xcframework \
        -library target/universal-ios/debug/$(STATIC_LIB_NAME) -headers target/bindings/headers \
		-library target/universal-mac/debug/$(STATIC_LIB_NAME) -headers target/bindings/headers \
		-library target/aarch64-apple-ios/debug/$(STATIC_LIB_NAME) -headers target/bindings/headers \
		-output target/RustFramework.xcframework

swift-framework:
	rm -rf target/Greeter.package || echo "skip removing"
	mkdir -p target/Greeter/Sources/Greeter
	mv target/RustFramework.xcframework target/Greeter/
	cp src/Package.swift target/Greeter/
	cp target/bindings/sources/*.swift target/Greeter/Sources/Greeter/
