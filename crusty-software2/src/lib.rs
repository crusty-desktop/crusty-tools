pub mod packages;
mod packages2;
pub mod utils;

pub mod prelude {
    pub use crate::packages::*;
    pub use crate::utils::get_config;
    pub use crate::utils::install_color_eyre;
    pub use crate::utils::run;
}
