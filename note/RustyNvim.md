# Rust and Nvim

This is my environment configuration with `nvim` for `rust` development. I started with the following guides:

- [Rust and Neovim - A Thorough Guide and Walkthrough](https://rsdlt.github.io/posts/rust-nvim-ide-guide-walkthrough-development-debug/)
- [YouTube video by `Let's get rusty`](https://www.youtube.com/watch?v=E2mKJ73M9pg&t=162s)

My learning environment is Ubuntu and Mac OS.

I installed the following components:

- [NeoVim](https://neovim.io/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Nerd Font](https://www.nerdfonts.com/#home)
  - You need to do this configuration for the terminal on your local machine, not the remote machine!
  - Download a font from [this list](https://www.nerdfonts.com/font-downloads)
    - I chose the Sauce CodePro Nerd Font
  - Unzip the contents to `~/.local/share/fonts`
  - Run `fc-cache -fv` to build font cache so the new font will be recognized.
  - Open a terminal and right click to select `Preference`. Select `Custom Font` under `Text`. You might need to restart the terminal so that font change takes effect.
  - If you are on a Mac OS, you will need `Font Book` to install fonts rather than running the `fc-cache` command.
  - To check whether the installation is successful, find an icon [here](https://www.nerdfonts.com/cheat-sheet), e.g., the wifi icon. Copy the icon code and paste it into your terminal. If it shows up, it worked.
- [NvChad](https://nvchad.com/docs/quickstart/install)
  - You might want to delete old nvim folders if necessary.
  - Do not forget to complete the post-installation steps: `:MasonInstallAll` and removing `.git`
  - TODO: `npm` is not an executable which failed the `html-lsp` and `css-lsp` installation.
- For debugger and inline information
  - [Rust Analyzer](https://rust-analyzer.github.io/) with `MasonInstall`
  - [rustaceanvim](https://github.com/mrcjkb/rustaceanvim) to make the rust analyzer work with neovim
  - [codelldb](https://github.com/vadimcn/codelldb) with `MasonInstall`
  - [nvim-dap](https://github.com/mfussenegger/nvim-dap) so that `nvim` can work with `lldb`
- [rust vim](https://github.com/rust-lang/rust.vim) for automatic formatting on save for rust files

