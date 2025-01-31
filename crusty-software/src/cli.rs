use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Config location file or directory
    #[arg(
        short,
        long,
        value_name = "FILE|DIR",
        value_hint = clap::ValueHint::AnyPath
    )]
    pub config: Option<PathBuf>,
    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
    /// Dry run
    #[arg(short, long)]
    pub dry_run: bool,
    /// Keep running if a step failed
    #[arg(short, long)]
    pub keep: bool,
}
