use crate::utils::{get_config, install_color_eyre, run};
use std::path::PathBuf;
use clap::Parser;
use color_eyre::Result;
use crate::packages::{Package, PackageList, PackageType};

mod packages;
mod utils;

const APP_NAME: &'static str = "crusty";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CommandLineArgs {
    /// Set's a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main()-> Result<()> {
    install_color_eyre()?;
    pretty_env_logger::init();
    let cli = CommandLineArgs::parse();

    let file = if let Some(file) = cli.config.as_deref() {
        log::info!("Reading config from {}", &file.display());
        file
    } else {
        let file = get_config(APP_NAME, "software.toml")?;
        log::info!("Reading default config from {}", &file.display());
        &file.clone()
    };

    if !file.is_file() && !file.exists() {
        eprintln!("Error: Config file {} is missing", file.to_string_lossy());
        std::process::exit(66);
    }
    let software_list: PackageList = toml::from_str(&std::fs::read_to_string(&file)?)
        .expect(&format!("failed to serialize {}", &file.display()));

    install_all_software(&software_list)?;

    Ok(())
}

fn install_all_software(list: &PackageList) -> Result<()> {
    let mut status = Vec::new();
    for (name, package) in list.iter() {
        for dep in &package.dependencies {
            if !list.contains_key(dep) {
                eprintln!("Error: Unknown dependency {} for {} ", dep, name);
                std::process::exit(66);
            }
            let dep_package = &list[dep];
            if !status.contains(dep) {
                install_package(dep, dep_package)?;
                status.push(dep.clone());
            }
        }
        if !status.contains(name) {
            install_package(name, package)?;
            status.push(name.clone());
        }
    }
    Ok(())
}

fn install_package(name: &str, package: &Package) -> Result<()> {
    let desc = package.description.clone().unwrap_or_default();
    log::trace!("Installing {} {}", name, desc);
    println!("Installing {} {}", name, desc);
    match package.package_type {
        PackageType::Flatpak => {}
        PackageType::System => {
            run(&vec!["sudo", "apt-get", "install", &package.source])?;
        }
        PackageType::Rust => {}
        PackageType::Custom => {}
    }
    Ok(())
}