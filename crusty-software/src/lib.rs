pub mod custom_package;
pub mod flatpak_package;
pub mod package_list;
pub mod provider;
pub mod rust_package;
pub mod system_package;

pub mod utils;

pub mod cli;

pub const APP_NAME: &str = "crusty";

pub mod prelude {
    pub use crate::custom_package::{CustomPackage, CustomPackageList};
    pub use crate::flatpak_package::{FlatpakPackage, FlatpakPackageList};
    pub use crate::package_list::PackageList;
    pub use crate::provider::{AliasList, CommonOptions, InstallOptions, PackageProvider};
    pub use crate::rust_package::{RustPackage, RustPackageList};
    pub use crate::system_package::{SystemPackage, SystemPackageList};
}
