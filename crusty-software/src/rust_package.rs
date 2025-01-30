use crate::prelude::{CommonOptions, PackageProvider};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RustPackage {
    #[serde(skip)]
    pub source: String,
    #[serde(default, flatten)]
    pub common_options: CommonOptions,
}

impl PackageProvider for RustPackage {
    fn get_source(&self) -> &str {
        self.source.as_str()
    }

    fn package_type(&self) -> &str {
        "rust"
    }
    fn get_extras(&self) -> &CommonOptions {
        &self.common_options
    }
    fn install_args(&self) -> Vec<&str> {
        vec!["cargo", "binstall", "-y", &self.source]
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
