use crate::prelude::{CommonOptions, PackageProvider};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CustomPackage {
    #[serde(skip)]
    pub source: String,
    #[serde(default)]
    pub script: String,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default, flatten)]
    pub common_options: CommonOptions,
}

impl PackageProvider for CustomPackage {
    fn get_source(&self) -> &str {
        self.source.as_str()
    }

    fn package_type(&self) -> &str {
        "Custom"
    }

    fn get_extras(&self) -> &CommonOptions {
        &self.common_options
    }

    fn install_args(&self) -> Vec<&str> {
        vec!["sh", "-c", &self.script]
    }

    fn check_if_installed(&self) -> bool {
        if let Some(cmd) = &self.command {
            which::which(cmd).is_ok()
        } else {
            false
        }
    }
}

impl CustomPackage {}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CustomPackageList(pub IndexMap<String, CustomPackage>);

impl CustomPackageList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl Deref for CustomPackageList {
    type Target = IndexMap<String, CustomPackage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CustomPackageList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
