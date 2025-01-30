use crate::prelude::{CommonOptions, PackageProvider};
use crate::provider::AliasList;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::process::Command;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FlatpakPackage {
    #[serde(skip)]
    pub source: String,
    #[serde(default, flatten)]
    pub common_options: CommonOptions,
}

impl PackageProvider for FlatpakPackage {
    fn get_source(&self) -> &str {
        self.source.as_str()
    }

    fn package_type(&self) -> &str {
        "flatpak"
    }

    fn get_extras(&self) -> &CommonOptions {
        &self.common_options
    }

    fn install_args(&self) -> Vec<&str> {
        vec!["flatpak", "install", "-y", &self.source]
    }

    fn get_aliases(&self) -> &AliasList {
        &self.common_options.alias
    }

    fn check_if_installed(&self) -> bool {
        let output = Command::new("flatpak").arg("list").output();
        if let Ok(output) = output {
            let output = String::from_utf8_lossy(&output.stdout);
            return output.contains(&self.source);
        }
        false
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FlatpakPackageList(pub IndexMap<String, FlatpakPackage>);

impl FlatpakPackageList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl Deref for FlatpakPackageList {
    type Target = IndexMap<String, FlatpakPackage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FlatpakPackageList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
