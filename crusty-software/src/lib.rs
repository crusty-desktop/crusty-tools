pub mod custom_package;
pub mod flatpak_package;
pub mod provider;
pub mod rust_package;
pub mod system_package;
pub mod utils;

pub const APP_NAME: &str = "crusty";

pub mod prelude {
    pub use crate::provider::*;
    pub use crate::rust_package::*;
}
