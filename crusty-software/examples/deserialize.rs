use crusty_software::prelude::{InstallOptions, PackageList};

const EXAMPLE: &str = r#"
[apt.mc]
source = "Description"
mc1 = "mc -f"
mc2 = "mc -g"

[flatpak."com.bitwarden.desktop"]

[apt.vim]
vim1 = "vim -f"
vim22 = "vim -g"

[rust.default1]

[apt.default2]

[rust.topgrade]

[rust.starship]

[flatpak."me.iepure.devtoolbox"]
"#;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let list = PackageList::deserialize(&EXAMPLE)?;

    let install_option = InstallOptions { verbose: true };
    list.install(&install_option)?;

    Ok(())
}
