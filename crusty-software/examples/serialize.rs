use crusty_software::custom_package::{CustomPackage, CustomPackageList};
use crusty_software::flatpak_package::FlatpakPackageList;
use crusty_software::prelude::*;
use crusty_software::system_package::{SystemPackage, SystemPackageList};
use indexmap::IndexMap;

fn main() {
    let package1 = SystemPackage {
        common_options: CommonOptions {
            alias: AliasList::from([
                ("mc1".to_string(), "mc -f".to_string()),
                ("mc2".to_string(), "mc -g".to_string()),
            ]),
            repository: Some("Some repository".to_string()),
            description: None,
            documentation: None,
        },
        ..Default::default()
    };

    let system_package_list = SystemPackageList(IndexMap::from([
        ("mc".to_string(), package1),
        ("default1".to_string(), SystemPackage::default()),
        ("default2".to_string(), SystemPackage::default()),
    ]));

    let rust_package_list = RustPackageList(IndexMap::from([]));
    let flatpak_list = FlatpakPackageList(IndexMap::from([]));

    let custom_list = CustomPackageList {
        0: IndexMap::from([(
            "lazygit".to_string(),
            CustomPackage {
                source: "".to_string(),
                common_options: Default::default(),
                script: r#"
LAZYGIT_VERSION=$(curl -s "https://api.github.com/repos/jesseduffield/lazygit/releases/latest" | \grep -Po '"tag_name": *"v\K[^"]*')
curl -Lo lazygit.tar.gz "https://github.com/jesseduffield/lazygit/releases/download/v${LAZYGIT_VERSION}/lazygit_${LAZYGIT_VERSION}_Linux_x86_64.tar.gz"
tar xf lazygit.tar.gz lazygit
sudo install lazygit -D -t /usr/local/bin/
                "#
                .to_string(),
            },
        )]),
    };

    let package_list = PackageList {
        apt: system_package_list,
        rust: rust_package_list,
        flatpak: flatpak_list,
        custom: custom_list,
    };

    // TODO package_list.serialize.rs()
    let toml = package_list.serialize().unwrap();
    println!("{}", toml);
}
