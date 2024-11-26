use crate::conf::Configuration;
use anyhow::Result;
use camino_fs::Utf8PathExt;

pub fn generate(conf: &Configuration, resource_dirs: &[&str]) -> Result<()> {
    let package_name = &conf.cargo_section.package_name;

    let resources = resource_dirs
        .iter()
        .map(resource_string)
        .collect::<Vec<_>>()
        .join("\n");

    let contents = format!(
        r###"import Foundation

extension {package_name} {{
{resources}
}}  "###,
    );

    let dir = conf.framework_build_dir.join("Sources").join(package_name);
    dir.mkdirs()?;
    dir.join("ResourcesExt.swift").write(contents)?;
    Ok(())
}

fn resource_string(name: &&str) -> String {
    format!("    public static let {name} = Bundle.module.url(forResource: \"{name}\", withExtension: nil)!")
}
