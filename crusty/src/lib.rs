use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub mod prelude {
    pub use crate::{Package, PackageCategory, PackageList, PackageType};
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum PackageType {
    Flatpak,
    System,
    Rust,
    Custom,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum PackageCategory {
    Library,
    Terminal,
    Gnome,
    Cosmic,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub description: Option<String>,
    pub source: String,
    #[serde(rename = "type")]
    pub package_type: PackageType,
    #[serde(rename = "category")]
    pub package_category: PackageCategory,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
    #[serde(default = "default_false", skip_serializing_if = "is_false")]
    pub optional: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]

pub struct PackageList {
    pub pkgs: IndexMap<String, Package>,
}

fn default_false() -> bool {
    false
}
fn is_false(value: &bool) -> bool {
    !*value
}
