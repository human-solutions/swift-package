fn main() {
    let metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();
    let uniffi_bindgen_version = metadata
        .packages
        .iter()
        .find(|pack| pack.name == "uniffi_bindgen")
        .unwrap()
        .version
        .to_string();

    println!("cargo::rustc-env=UNIFFY_BINDGEN_VERSION={uniffi_bindgen_version}")
}
