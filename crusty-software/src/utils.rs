use crate::prelude::InstallOptions;
use color_eyre::eyre::eyre;
use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn run(args: &[&str], options: &InstallOptions) -> Result<()> {
    let program = args.get(0).ok_or_else(|| eyre!("no program specified"))?;
    if options.dry_run {
        println!("- Will run {}", args.join(" "));
        return Ok(());
    }

    let program_path =
        which::which(&program).map_err(|_| eyre!("failed to find executable {}", program))?;
    let mut command = Command::new(program_path);
    command.args(&args[1..]);

    if options.verbose {
        let mut child = command.spawn()?;
        let output = child.wait_with_output()?;

        if !output.status.success() {
            let exit_code = output.status.code().expect("no exit status");
            return Err(eyre!("Command execution failed"));
        }
    } else {
        let _ = command.output()?;
    }
    Ok(())
}

/// Set up the error handler crate
pub fn install_color_eyre() -> Result<()> {
    color_eyre::config::HookBuilder::new()
        .display_env_section(false)
        .display_location_section(true)
        .install()
}

pub fn get_config(app: &str, file: &str) -> Result<PathBuf> {
    Ok(dirs::config_dir()
        .context("Failed to get config directory")?
        .join(app)
        .join(file))
}
