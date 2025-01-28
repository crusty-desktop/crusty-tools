use clap::Parser;
use color_eyre::Result;
use console::{style, Emoji};
use crusty_software::prelude::*;
use std::path::PathBuf;

const APP_NAME: &'static str = "crusty";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CommandLineArgs {
    /// Set's a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");

fn main() -> Result<()> {
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
    println!(
        "{} Installing {} {}",
        TRUCK,
        style(name).bold().dim().green(),
        desc
    );
    log::info!("Installing {} {}", name, desc);
    match package.package_type {
        PackageType::Flatpak => {
            install_flatpak(&name, &package)?;
        }
        PackageType::System => {
            install_apt(&name, &package)?;
        }
        PackageType::Rust => {
            install_rust(&name, &package)?;
        }
        PackageType::Custom => {
            todo!()
        }
        PackageType::Library => {
            install_lib(&name, &package)?;
        }
    }

    // TODO: Append alias to $config/alias.sh (if not exist, make file if not exist)
    for (name, alias) in &package.alias {
        install_alias(name, alias);
    }
    Ok(())
}

fn install_alias(name: &String, alias: &String) {
    println!("{}=> {}", name, alias);
}

fn install_rust(name: &str, package: &&Package) -> Result<()> {
    if test_which(&name, package) {
        return Ok(());
    }
    let args = vec!["cargo", "binstall", "-y", &package.source];
    log::trace!("Exec: {name} {}", args.join(" "));
    run(&args)?;
    Ok(())
}

fn install_lib(name: &str, package: &&Package) -> Result<()> {
    let args = vec!["cargo", "binstall", "-y", &package.source];
    log::trace!("Exec: {name} {}", args.join(" "));
    run(&args)?;
    Ok(())
}
fn install_apt(name: &str, package: &&Package) -> Result<()> {
    if test_which(&name, package) {
        return Ok(());
    }
    let args = vec!["sudo", "apt-get", "install", "-y", &package.source];
    log::trace!("Exec: {name} {}", args.join(" "));
    run(&args)?;
    Ok(())
}

fn install_flatpak(_name: &str, package: &&Package) -> Result<()> {
    // TODO: Check if flatpak is installed
    let args = vec!["flatpak", "install", "-y", &package.source];
    log::trace!("Exec: {}", args.join(" "));
    run(&args)?;
    Ok(())
}

fn test_which(name: &&str, package: &&Package) -> bool {
    if package.executable.is_some() {
        let executable = package.executable.clone().unwrap();
        if let Ok(p) = which::which(&executable) {
            let executable = p.display();
            log::info!("Skip installation of {name} as {executable} exists on path");
            return true;
        }
    } else {
        if let Ok(p) = which::which(&name) {
            let executable = p.display();
            log::info!("Skip installation of {name} as {executable} exists on path");
            return true;
        }
    }
    false
}
