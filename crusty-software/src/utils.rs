use std::path::{Path, PathBuf};
use std::process::Command;
use color_eyre::eyre::eyre;
use color_eyre::Result;


pub fn run(args: &[&str]) -> Result<()> {
    let program = args.get(0).ok_or_else(|| eyre!("no program specified"))?;
    let program_path = which::which(&program).map_err(|_| eyre!("failed to find sudo executable {}", program))?;
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

#[cfg(not(debug_assertions))]
pub fn get_config(app: &str, file: &str) -> Result<PathBuf> {
    Ok(dirs::config_dir()
        .context("Failed to get config directory")?
        .join(app)
        .join(file))
}
#[cfg(debug_assertions)]
pub fn get_config(_app: &str, file: &str) -> Result<PathBuf> {
    Ok(workspace_dir().join("support").join("settings").join(file))
}

#[cfg(debug_assertions)]
// https://stackoverflow.com/questions/43577885/is-there-a-cargo-environment-variable-for-the-workspace-directory
fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

