use crusty::prelude::*;

fn main() {
    let mut list = PackageList::default();
    list.pkgs.insert(
        "code".to_string(),
        Package {
            name: "code".to_string(),
            description: Some("VS Code".to_string()),
            package_type: PackageType::System,
            package_category: PackageCategory::Gnome,
            source: "code".to_string(),
            dependencies: vec!["xlibs".to_string()],
            optional: false,
        },
    );
    list.pkgs.insert(
        "subl".to_string(),
        Package {
            name: "subl".to_string(),
            description: None,
            package_type: PackageType::Flatpak,
            package_category: PackageCategory::Library,
            source: "subl-dev".to_string(),
            dependencies: vec![],
            optional: true,
        },
    );

    println!("====== Serialize ==========");
    let toml = toml::to_string_pretty(&list).unwrap();
    println!("{}", toml);

    let mut packages: PackageList = toml::from_str(&toml).unwrap();
    println!("====== De-Serialize ==========");
    let toml = toml::to_string_pretty(&packages).unwrap();
    println!("{}", toml);
}
