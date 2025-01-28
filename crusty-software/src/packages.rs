use crate::packages::PackageCategory::Terminal;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageType {
    Flatpak,
    System,
    /// Same as system but they don't have en executable to check
    Library,
    Rust,
    Custom,
}

impl Default for PackageType {
    fn default() -> Self {
        Self::Library
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum PackageCategory {
    #[serde(alias = "Library", alias = "library")]
    Library,
    #[serde(alias = "Terminal", alias = "terminal")]
    Terminal,
    #[serde(alias = "Gnome", alias = "gnome")]
    Gnome,
    #[serde(alias = "Cosmic", alias = "COSMIC", alias = "Cosmic")]
    Cosmic,
    #[serde(alias = "Gui", alias = "X11", alias = "gui")]
    Gui,
}

impl Default for PackageCategory {
    fn default() -> Self {
        Terminal
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub description: Option<String>,
    pub source: String,
    pub executable: Option<String>,
    #[serde(rename = "type", default = "PackageType::default")]
    pub package_type: PackageType,
    #[serde(rename = "category")]
    pub package_category: Option<PackageCategory>,
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
