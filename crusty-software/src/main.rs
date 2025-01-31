use clap::Parser;
use crusty_software::cli::Cli;
use crusty_software::prelude::*;
use crusty_software::utils::{get_config, install_color_eyre};
use crusty_software::APP_NAME;
use std::path::PathBuf;

fn main() -> color_eyre::Result<()> {
    install_color_eyre()?;

    let cli = Cli::parse();

    let options = InstallOptions {
        verbose: cli.verbose,
        dry_run: cli.dry_run,
        keep_running: false,
    };

    let config = if cli.config.is_none() {
        get_config(APP_NAME, "software.d")?
    } else {
        cli.config.unwrap()
    };

    if !config.exists() {
        // TODO: Nice reporting
        eprintln!("Error: Path {} not found on filesystem", config.display());
        report_install_instructions();
        std::process::exit(1);
    }

    if config.is_file() {
        // No check on filename here
        PackageList::install(&config, &options)?;
    } else {
        let mut file_lists = std::fs::read_dir(&config)?.collect::<Vec<_>>();
        if file_lists.is_empty() {
            eprintln!("Error: No config files found in {}", config.display());
            report_install_instructions();
            std::process::exit(1);
        }

        // Sort by file name
        file_lists.sort_by_key(|entry| {
            entry
                .as_ref()
                .ok()
                .map(|e| e.file_name())
                .map(|name| name.to_string_lossy().to_string())
        });

        for entry in file_lists {
            let path = entry?.path();
            if is_valid_filename(&path) {
                PackageList::install(&path, &options)?;
            }
        }
    }

    Ok(())
}

fn is_valid_filename(path: &PathBuf) -> bool {
    // Files must have extension '.toml' and filename must not start with '_' or '.'
    path.is_file()
        && path
            .file_name()
            .and_then(|n| n.to_str())
            .map_or(true, |f| !f.starts_with('_') && !f.starts_with('.'))
        && path.extension().and_then(|ext| ext.to_str()) == Some("toml")
}

fn report_install_instructions() {
    println!("TODO: How to install");
}
