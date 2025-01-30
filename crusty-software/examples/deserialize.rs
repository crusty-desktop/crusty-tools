use crusty_software::prelude::PackageList;

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

[custom.lazygit]
script = '''
LAZYGIT_VERSION=$(curl -s "https://api.github.com/repos/jesseduffield/lazygit/releases/latest" | \grep -Po '"tag_name": *"v\K[^"]*')
curl -Lo lazygit.tar.gz "https://github.com/jesseduffield/lazygit/releases/download/v${LAZYGIT_VERSION}/lazygit_${LAZYGIT_VERSION}_Linux_x86_64.tar.gz"
tar xf lazygit.tar.gz lazygit
sudo install lazygit -D -t /usr/local/bin/'''

[lazygit.alias]
git = "lazygit"
"#;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let list = PackageList::deserialize(&EXAMPLE)?;
    for package in list.apt.iter() {
        println!("{:#?}", package);
    }
    for package in list.custom.iter() {
        println!("{:#?}", package);
    }
    Ok(())
}
