use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageType {
    Flatpak,
    System,
    Rust,
    Custom,
}

impl Default for PackageType {
    fn default() -> Self {
        Self::System
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub description: Option<String>,
    pub source: Option<String>,
    pub executable: Option<String>,
    #[serde(rename = "type", default = "PackageType::default")]
    pub package_type: PackageType,
    #[serde(default, rename = "deps", skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub alias: IndexMap<String, String>,
    #[serde(default = "default_false", skip_serializing_if = "is_false")]
    pub optional: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]

pub struct PackageList {
    pub pkg: IndexMap<String, Package>,
}

impl Deref for PackageList {
    type Target = IndexMap<String, Package>;

    fn deref(&self) -> &Self::Target {
        &self.pkg
    }
}

impl DerefMut for PackageList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pkg
    }
}

fn default_false() -> bool {
    false
}

fn is_false(value: &bool) -> bool {
    !*value
}
