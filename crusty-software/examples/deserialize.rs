use crusty_software::prelude::{InstallOptions, PackageList};

const EXAMPLE: &str = r#"
[apt.mc]
repository = "Some repository"

[apt.mc.alias]
mc1 = "mc -f"
mc2 = "mc -g"

[rust.eza]
description = "A modern colorful ls"
lsa = "eza --icons --hyperlink --color-scale --group-directories-first  --all"

[rust.eza.alias]
ls = "eza --icons --hyperlink --color-scale --group-directories-first"
lst = 'eza --tree --level=2 --long --icons --git'
dir = "eza --icons --hyperlink -l -h --git"
dirs = "eza --icons --hyperlink -l -h --git --color-scale --total-size"

[flatpak."dev.edfloreshz.CosmicTweaks"]
repository = "https://github.com/cosmic-utils/tweaks"

[flatpak."dev.edfloreshz.CosmicTweaks".alias]
cosmic_tweaks = "flatpak run dev.edfloreshz.CosmicTweaks"

"#;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let list = PackageList::deserialize(&EXAMPLE)?;
    for package in list.apt.iter() {
        println!("{:#?}", package);
    }

    Ok(())
}
