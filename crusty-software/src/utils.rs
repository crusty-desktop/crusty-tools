use color_eyre::eyre::eyre;
use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn run(args: &[&str]) -> Result<()> {
    let program = args.get(0).ok_or_else(|| eyre!("no program specified"))?;
    let program_path =
        which::which(&program).map_err(|_| eyre!("failed to find sudo executable {}", program))?;
    let mut command = Command::new(program_path);
    command.args(&args[1..]);

    let child = command.spawn()?;
    let output = child.wait_with_output()?;

    if !output.status.success() {
        let exit_code = output.status.code().expect("no exit status");
        println!("Command failed with exit code: {}", exit_code);
        return Err(eyre!("Command execution failed"));
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
