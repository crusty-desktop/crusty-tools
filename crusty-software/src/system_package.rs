use crate::prelude::InstallOptions;
use crate::provider::AliasList;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SystemPackage {
    #[serde(skip)]
    pub source: String,
    #[serde(default, flatten, skip_serializing_if = "IndexMap::is_empty")]
    pub alias: AliasList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
}

impl SystemPackage {
    pub fn install(&self, options: &InstallOptions) -> color_eyre::Result<()> {
        println!("Installing: sudo apt install {}", self.source);
        Ok(())
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SystemPackageList(pub IndexMap<String, SystemPackage>);

impl SystemPackageList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl Deref for SystemPackageList {
    type Target = IndexMap<String, SystemPackage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SystemPackageList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
