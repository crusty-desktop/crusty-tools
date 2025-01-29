use crate::prelude::InstallOptions;
use crate::provider::{AliasList, PackageProvider};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FlatpakPackage {
    #[serde(skip)]
    pub source: String,
    #[serde(default, flatten, skip_serializing_if = "IndexMap::is_empty")]
    pub alias: AliasList,
}

impl FlatpakPackage {
    pub fn install(&self, options: &InstallOptions) -> color_eyre::Result<()> {
        println!("Installing: flatpak install {}", self.source);
        Ok(())
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
