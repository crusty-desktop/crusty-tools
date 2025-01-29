use crate::packages::Package;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

trait InstallListSourcecd {
    fn canonicalize(&mut self);
}

type AliasList = IndexMap<String, String>;
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct SystemPackage {
    source: Option<String>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    alias: AliasList,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct SystemPackageList(IndexMap<String, SystemPackage>);

impl Deref for SystemPackageList {
    type Target = IndexMap<String, SystemPackage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SystemPackageList {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct RustPackage {
    source: Option<String>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    alias: AliasList,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct RustPackageList(IndexMap<String, RustPackage>);

impl Deref for RustPackageList {
    type Target = IndexMap<String, RustPackage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RustPackageList {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct PackageList {
    #[serde(default, skip_serializing_if = "SystemPackageList::is_empty")]
    pkg: SystemPackageList,
    #[serde(default, skip_serializing_if = "RustPackageList::is_empty")]
    rust: RustPackageList,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default;

    const TEMPLATE_FILE: &str = r#"
    [pkg.mc]

    [pkg.vim]
    alias.vi="vim"

    [pkg.apkg]
    description="a description for ci"
    source="apkg_source"
    alias.alias1="apkg -l"
    alias.alias2="apkg -k"
    "#;

    #[test]
    fn test_serialize_system_package_list() {
        let system_package_list = SystemPackageList(IndexMap::from([
            ("mc".to_string(), SystemPackage::default()),
            ("vim".to_string(), SystemPackage::default()),
            (
                "apkg".to_string(),
                SystemPackage {
                    source: Some("apkg_source".to_string()),
                    alias: IndexMap::from([
                        ("alias1".to_string(), "apkg -l".to_string()),
                        ("alias2".to_string(), "apkg -k".to_string()),
                    ]),
                },
            ),
        ]));

        let package_list = PackageList {
            pkg: system_package_list,
            ..Default::default()
        };

        let toml = toml::to_string_pretty(&package_list);
        assert!(toml.is_ok());
        println!("{}", toml.unwrap());
    }
    #[test]
    fn test_deserialize_system_package() {
        let list = toml::from_str::<PackageList>(&TEMPLATE_FILE);
        println!("{:#?}", list);
        assert!(list.is_ok());
        let list = list.unwrap();
        assert_eq!(list.pkg.iter().len(), 3);
        let (name, package) = list.pkg.iter().last().unwrap();
        assert_eq!(name, "apkg");
        assert_eq!(package.source.as_ref().unwrap(), "apkg_source");
        assert_eq!(package.alias.len(), 2);
        assert_eq!(package.alias["alias1"], "apkg -l");
    }
}
