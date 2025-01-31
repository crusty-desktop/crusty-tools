use color_eyre::Result;
use ctrem::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
pub type AliasList = IndexMap<String, String>;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommonOptions {
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub alias: AliasList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
}

pub trait PackageProvider {
    fn get_source(&self) -> &str;

    fn package_type(&self) -> &str;

    fn get_extras(&self) -> &CommonOptions;

    fn install_args(&self) -> Vec<&str>;

    fn get_aliases(&self) -> &AliasList {
        &self.get_extras().alias
    }

    fn check_if_installed(&self) -> bool {
        false
    }

    fn install(&self, options: &InstallOptions) -> Result<()> {
        if self.check_if_installed() {
            cprintln(&format!(
                "   [blue]-  Skip[/] installing {}: [green]{}[/]",
                self.package_type(),
                self.get_source()
            ));
            return Ok(());
        }
        self.installing_header();

        match crate::utils::run(&self.install_args(), options) {
            Ok(_) => {}
            Err(_err) => {
                cprintln(&format!(
                    "   [red]-  Error[/] installing {}: [green]{}[/]",
                    self.package_type(),
                    self.get_source()
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
            "   [green]+[/] Installing [blue]{}[/] package [blue]{}[/]",
            self.package_type(),
            self.get_source()
        ));
        if let Some(description) = &self.get_extras().description {
            println!("     {}", description);
        }
        if let Some(text) = &self.get_extras().repository {
            println!("     - Repository: {}", text);
        }
        if let Some(text) = &self.get_extras().documentation {
            println!("     - Documentation: {}", text);
        }
    }
}

pub trait PackageListProvider {}

#[derive(Debug, Clone)]
pub struct InstallOptions {
    pub verbose: bool,
    pub dry_run: bool,
    pub keep_running: bool,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            verbose: true,
            dry_run: false,
            keep_running: false,
        }
    }
}
