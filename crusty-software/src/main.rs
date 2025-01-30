use clap::Parser;
use crusty_software::prelude::{InstallOptions, PackageList};
use crusty_software::utils::{get_config, install_color_eyre};
use crusty_software::APP_NAME;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CommandLineArgs {
    /// Config location file or directory
    #[arg(
        short,
        long,
        value_name = "FILE|DIR",
        value_hint = clap::ValueHint::AnyPath
    )]
    config: Option<PathBuf>,
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    /// Dry run
    #[arg(short, long)]
    dry_run: bool,
    /// Keep running if a step failed
    #[arg(short, long)]
    keep: bool,
}

fn main() -> color_eyre::Result<()> {
    install_color_eyre()?;

    let cli = CommandLineArgs::parse();

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
        std::process::exit(1);
    }

    if config.is_file() {
        // No check on filename here
        PackageList::install(&config, &options)?;
    } else {
        for entry in std::fs::read_dir(&config)? {
            let entry = entry?;
            let path = entry.path();
            // Files must have extension '.toml' and filename must not start with '_' or '.'
            if path.is_file()
                && path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map_or(true, |f| !f.starts_with('_') && !f.starts_with('.'))
                && path.extension().and_then(|ext| ext.to_str()) == Some("toml")
            {
                PackageList::install(&path, &options)?;
            }
        }
    }

    Ok(())
}
