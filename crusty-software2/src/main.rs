use clap::Parser;
use color_eyre::Result;
// use console::{style, Emoji};
use crusty_software2::prelude::*;
use indexmap::IndexMap;
use std::path::PathBuf;
use toml::de::Error;

const APP_NAME: &'static str = "crusty";
const TEMPLATE_FILE: &str = include_str!("software-template.toml");
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CommandLineArgs {
    /// Set's a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

// static TRUCK: Emoji<'_, '_> = Emoji("🚚 ", "");

fn main() -> Result<()> {
    install_color_eyre()?;
    pretty_env_logger::init();

    create_conf_dir_structure()?;

    let cli = CommandLineArgs::parse();

    let mut config_files: Vec<PathBuf> = Vec::new();
    if cli.config.is_some() {
        log::trace!("No config file specified, using default settings.");
        config_files.push(cli.config.unwrap());
    } else {
        let settings_dir = get_config(APP_NAME, "software.d")?;
        for entry in std::fs::read_dir(settings_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("toml") {
                config_files.push(path);
            }
        }
    }

    if config_files.is_empty() {
        eprintln!("Error: No configuration files found.");
        std::process::exit(1);
    }

    // Merge configs
    let mut merged_packages = indexmap::IndexMap::new();
    for config_file in config_files {
        let toml_text = std::fs::read_to_string(&config_file)?;
        match toml::from_str::<PackageList>(&toml_text) {
            Ok(list) => {
                log::info!("Loading config file {}", config_file.display());
                merge_lists(&mut merged_packages, list);
            }
            Err(err) => {
                log::error!("Error parsing config file {}", config_file.display());
                print_toml_parsing_error(config_file, toml_text, err);
            }
        }
    }

    for (name, package) in merged_packages.iter() {
        println!(
            "{} => {:?}, {:?}",
            name, package.source, package.package_type
        );
    }

    // let file = if let Some(file) = cli.config {
    //     log::info!("Reading config from {}", &file.display());
    //     file
    // } else {
    //     let file = get_config(APP_NAME, "software.toml")?;
    //     log::info!("Reading default config from {}", &file.display());
    //     file
    // };
    //
    // if !file.is_file() && !file.exists() {
    //     eprintln!("Error: Config file {} is missing", file.to_string_lossy());
    //     std::process::exit(66);
    // }
    //
    // let software_list: PackageList = toml::from_str(&std::fs::read_to_string(&file)?)
    //     .expect(&format!("failed to serialize.rs {}", &file.display()));
    //
    // install_all_software(&software_list)?;

    Ok(())
}

fn merge_lists(merged_packages: &mut IndexMap<String, Package>, list: PackageList) {
    for (name, package) in list.iter() {
        if merged_packages.contains_key(name) {
            let original = merged_packages.get(name).unwrap();
            if original != package {
                eprintln!(
                    "Waring: Duplicate package name: {}, first entry wins.",
                    name
                );
            }
        } else {
            merged_packages.insert(name.clone(), package.clone());
        }
    }
}

fn print_toml_parsing_error(config_file: PathBuf, toml_text: String, err: Error) {
    eprintln!("TOML Parsing Error: {}", config_file.display());
    eprintln!("{}", err.message());
    if let Some(span) = err.span() {
        println!("span {:?}", span);
        let (start, _end) = (span.start, span.end);
        // let snippet = &toml_text[start..end];
        // println!("Fix this: {}", snippet);
        let lines = toml_text.lines().collect::<Vec<&str>>();
        let start_line_index = toml_text[..start].lines().count();
        if start_line_index < lines.len() {
            let line_number = start_line_index + 1;
            eprintln!("{}: {}", line_number, lines[start_line_index]);
        }
    }
}

fn create_conf_dir_structure() -> Result<()> {
    // Create settings dir and config file
    let settings_dir = get_config(APP_NAME, "software.d")?;
    if !settings_dir.exists() {
        log::info!("Creating config directory {}", &settings_dir.display());
        std::fs::create_dir_all(&settings_dir)?;
    }

    let default_settings_file = settings_dir.join("software.toml");
    if !default_settings_file.exists() {
        log::info!(
            "Creating default settings file {}",
            &default_settings_file.display()
        );
        std::fs::write(default_settings_file, TEMPLATE_FILE)?;
    }
    Ok(())
}

// fn install_all_software(list: &PackageList) -> Result<()> {
//     let mut status = Vec::new();
//     for (name, package) in list.iter() {
//         for dep in &package.dependencies {
//             if !list.contains_key(dep) {
//                 eprintln!("Error: Unknown dependency {} for {} ", dep, name);
//                 std::process::exit(66);
//             }
//             let dep_package = &list[dep];
//             if !status.contains(dep) {
//                 install_package(dep, dep_package)?;
//                 status.push(dep.clone());
//             }
//         }
//         if !status.contains(name) {
//             install_package(name, package)?;
//             status.push(name.clone());
//         }
//     }
//     Ok(())
// }
//
// fn install_package(name: &str, package: &Package) -> Result<()> {
//     let desc = package.description.clone().unwrap_or_default();
//     println!(
//         "{} Installing {} {}",
//         TRUCK,
//         style(name).bold().dim().green(),
//         desc
//     );
//     log::info!("Installing {} {}", name, desc);
//     match package.package_type {
//         PackageType::Flatpak => {
//             install_flatpak(&name, &package)?;
//         }
//         PackageType::System => {
//             install_apt(&name, &package)?;
//         }
//         PackageType::Rust => {
//             install_rust(&name, &package)?;
//         }
//         PackageType::Custom => {
//             todo!()
//         }
//         PackageType::Library => {
//             install_lib(&name, &package)?;
//         }
//     }
//
//     // TODO: Append alias to $config/alias.sh (if not exist, make file if not exist)
//     for (name, alias) in &package.alias {
//         install_alias(name, alias);
//     }
//     Ok(())
// }
//
// fn install_alias(name: &String, alias: &String) {
//     println!("{}=> {}", name, alias);
// }
//
// fn install_rust(name: &str, package: &Package) -> Result<()> {
//     if test_which(&name, package) {
//         return Ok(());
//     }
//     let package = package.clone();
//     let source = package.source.ok_or_else(|| name).unwrap();
//     let args = vec!["cargo", "binstall", "-y", &source];
//     log::trace!("Exec: {name} {}", args.join(" "));
//     run(&args)?;
//     Ok(())
// }
//
// fn install_lib(name: &str, package: &Package) -> Result<()> {
//     let package = package.clone();
//     let source = package.source.ok_or_else(|| name.to_string()).unwrap();
//     let args = vec!["sudo", "apt-get", "install", "-y", &source];
//     log::trace!("Exec: {name} {}", args.join(" "));
//     run(&args)?;
//     Ok(())
// }
// fn install_apt(name: &str, package: &Package) -> Result<()> {
//     if test_which(&name, &package) {
//         return Ok(());
//     }
//     let package = package.clone();
//     let source = package.source.ok_or_else(|| name.to_string()).unwrap();
//
//     let args = vec!["sudo", "apt-get", "install", "-y", &source];
//     log::trace!("Exec: {name} {}", args.join(" "));
//     run(&args)?;
//     Ok(())
// }
//
// fn install_flatpak(name: &str, package: &Package) -> Result<()> {
//     let package = package.clone();
//     let source = package.source.ok_or_else(|| name.to_string()).unwrap();
//     // TODO: Check if flatpak is installed
//     let args = vec!["flatpak", "install", "-y", &source];
//     log::trace!("Exec: {}", args.join(" "));
//     run(&args)?;
//     Ok(())
// }
//
// fn test_which(name: &&str, package: &Package) -> bool {
//     if package.executable.is_some() {
//         let executable = package.executable.clone().unwrap();
//         if let Ok(p) = which::which(&executable) {
//             let executable = p.display();
//             log::info!("Skip installation of {name} as {executable} exists on path");
//             return true;
//         }
//     } else {
//         if let Ok(p) = which::which(&name) {
//             let executable = p.display();
//             log::info!("Skip installation of {name} as {executable} exists on path");
//             return true;
//         }
//     }
//     false
// }
