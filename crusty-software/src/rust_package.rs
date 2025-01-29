use crate::prelude::InstallOptions;
use crate::provider::{AliasList, PackageProvider};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RustPackage {
    #[serde(skip)]
    pub source: String,
    #[serde(default, flatten, skip_serializing_if = "IndexMap::is_empty")]
    pub alias: AliasList,
}

impl RustPackage {
    pub fn install(&self, options: &InstallOptions) -> color_eyre::Result<()> {
        println!("Installing: cargo binstall {}", self.source);
        Ok(())
    }
}
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RustPackageList(pub IndexMap<String, RustPackage>);

impl RustPackageList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl Deref for RustPackageList {
    type Target = IndexMap<String, RustPackage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RustPackageList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_package_creation() {
//         let default = RustPackage::default();
//         assert!(default.source.is_none());
//         assert!(default.alias.is_empty());
//         println!("{:?}", default);
//         let with_name = RustPackage {
//             source: Some("mc".to_string()),
//             ..Default::default()
//         };
//         assert_eq!(with_name.source.unwrap(), "mc");
//         assert!(with_name.alias.is_empty());
//         let package = RustPackage {
//             source: Some("mc".to_string()),
//             alias: AliasList::from([
//                 ("mc1".to_string(), "mc -f".to_string()),
//                 ("mc2".to_string(), "mc -g".to_string()),
//             ]),
//         };
//         assert_eq!(package.source.unwrap(), "mc");
//         assert_eq!(package.alias.len(), 2);
//         assert_eq!(package.alias["mc1"], "mc -f");
//     }
// }
