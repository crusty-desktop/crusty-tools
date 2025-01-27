use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageType {
    Flatpak,
    System,
    Rust,
    Custom,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageCategory {
    Library,
    Terminal,
    Gnome,
    Cosmic,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub description: Option<String>,
    pub source: String,
    #[serde(rename = "type")]
    pub package_type: PackageType,
    #[serde(rename = "category")]
    pub package_category: PackageCategory,
    #[serde(default, rename = "deps", skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
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

fn default_false() -> bool {
    false
}

fn is_false(value: &bool) -> bool {
    !*value
}
