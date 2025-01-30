use crusty_software::flatpak_package::FlatpakPackageList;
use crusty_software::prelude::*;
use crusty_software::system_package::{SystemPackage, SystemPackageList};
use indexmap::IndexMap;

fn main() {
    let package1 = SystemPackage {
        alias: AliasList::from([
            ("mc1".to_string(), "mc -f".to_string()),
            ("mc2".to_string(), "mc -g".to_string()),
        ]),
        common_options: CommonOptions {
            alias: Default::default(),
            repository: Some("Some repository".to_string()),
            description: None,
            documentation: None,
        },
        ..Default::default()
    };

    let package2 = SystemPackage {
        alias: AliasList::from([
            ("vim1".to_string(), "vim -f".to_string()),
            ("vim22".to_string(), "vim -g".to_string()),
        ]),
        ..Default::default()
    };

    let system_package_list = SystemPackageList(IndexMap::from([
        ("mc".to_string(), package1),
        ("dev.edfloreshz.CosmicTweaks".to_string(), package2),
        ("default1".to_string(), SystemPackage::default()),
        ("default2".to_string(), SystemPackage::default()),
    ]));

    let rust_package_list = RustPackageList(IndexMap::from([]));
    let flatpak_list = FlatpakPackageList(IndexMap::from([]));

    let package_list = PackageList {
        apt: system_package_list,
        rust: rust_package_list,
        flatpak: flatpak_list,
    };

    // TODO package_list.serialize.rs()
    let toml = package_list.serialize().unwrap();
    println!("{}", toml);
}
