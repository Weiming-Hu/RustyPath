# Rust and Nvim

This is my environment configuration with `nvim` for `rust` development. I started with this guide: [Rust and Neovim - A Thorough Guide and Walkthrough](https://rsdlt.github.io/posts/rust-nvim-ide-guide-walkthrough-development-debug/).

- [NeoVim](https://neovim.io/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Nerd Font](https://www.nerdfonts.com/#home)
  - You need to do this configuration for the terminal on your local machine, not the remote machine!
  - Download a font from [this list](https://www.nerdfonts.com/font-downloads)
    - I chose the Sauce CodePro Nerd Font
  - Unzip the contents to `~/.local/share/fonts`
  - Run `fc-cache -fv` to build font cache so the new font will be recognized.
  - Open a terminal and right click to select `Preference`. Select `Custom Font` under `Text`. You might need to restart the terminal so that font change takes effect.
  - To check whether the installation is successful, find an icon [here](https://www.nerdfonts.com/cheat-sheet), e.g., the wifi icon. Copy the icon code and paste it into your terminal. If it shows up, it worked.
- [NvChad](https://nvchad.com/docs/quickstart/install)
  - You might want to delete old nvim folders if necessary.
  - I chose this because it does not require `lua`.
  - Do not forget to complete the post-installation steps: `:MasonInstallAll` and removing `.git`
- 
