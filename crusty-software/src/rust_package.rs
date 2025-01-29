use crate::prelude::InstallOptions;
use crate::provider::AliasList;
use ctrem::cprintln;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RustPackage {
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

impl RustPackage {
    pub fn install(&self, options: &InstallOptions) -> color_eyre::Result<()> {
        self.installing_header();
        let args = vec!["cargo", "binstall", "-y", &self.source];
        match crate::utils::run(&args, options) {
            Ok(_) => {}
            Err(_err) => {
                cprintln(&format!(
                    "   [red]-  Error[/] installing package: [green]{}[/]",
                    self.source
                ));
                if !options.keep_running {
                    std::process::exit(1);
                }
            }
        }
        Ok(())
    }

    fn installing_header(&self) {
        cprintln(&format!(
            "   [green]+[/] Installing rust package [blue]{}[/]",
            self.source
        ));
        if let Some(description) = &self.description {
            println!("     {}", description);
        }
        if let Some(text) = &self.repository {
            println!("     - Repository: {}", text);
        }
        if let Some(text) = &self.documentation {
            println!("     - Documentation: {}", text);
        }
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
