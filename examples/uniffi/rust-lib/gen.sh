cargo build --lib
rm -rf out
cargo run  --features=uniffi/cli --bin uniffi-bindgen generate src/lib.udl --language swift --out-dir out --no-format
sed -i '' 's/module\ MathFFI/framework\ module\ MathFFI/' out/MathFFI.modulemap
