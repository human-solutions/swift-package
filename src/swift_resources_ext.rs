use crate::conf::Configuration;
use anyhow::Result;
use camino_fs::Utf8PathExt;

pub fn generate(conf: &Configuration, resource_dirs: &[&str]) -> Result<()> {
    let package_name = &conf.cargo_section.package_name;

    let mut code = vec!["import Foundation\n".to_string()];
    code.extend(resource_dirs.iter().map(resource_string));

    let dir = conf.framework_build_dir.join("Sources").join(package_name);
    dir.mkdirs()?;
    dir.join("Resources.swift").write(code.join(""))?;
    Ok(())
}

fn resource_string(func: &&str) -> String {
    format!(
        r#"
public func {func}(name: String) -> URL {{
    return Bundle.module.url(forResource: "{func}", withExtension: nil)!
        .appendingPathComponent(name)
}}
"#
    )
}
