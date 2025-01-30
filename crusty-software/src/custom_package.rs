use crate::prelude::{CommonOptions, PackageProvider};
use crate::provider::AliasList;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CustomPackage {
    #[serde(skip)]
    pub source: String,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub alias: AliasList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
}

impl PackageProvider for CustomPackage {
    fn get_source(&self) -> &str {
        todo!()
    }

    fn get_aliases(&self) -> &AliasList {
        &self.alias
    }

    fn package_type(&self) -> &str {
        "Custom"
    }

    fn get_extras(&self) -> &CommonOptions {
        todo!()
    }

    fn install_args(&self) -> Vec<&str> {
        todo!()
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
