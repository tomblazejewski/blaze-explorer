A neovim-inspired, non-async file explorer, written purely in Rust.

# App Mappings

## Normal mode

| Mappings      | Action                                                                                          |
| ------------- | ----------------------------------------------------------------------------------------------- |
| `j`           | Next item                                                                                       |
| `k`           | Previous item                                                                                   |
| `<Backspace>` | Move to parent directory                                                                        |
| `<Enter>`     | Open selected item                                                                              |
| `dd`          | Delete selected item                                                                            |
| `r`           | Rename selected item                                                                            |
| `u`           | Undo last action (delete/rename)                                                                |
| `n`           | Next search result                                                                              |
| `N`           | Previous search result                                                                          |
| `<C-w>v`      | Split vertically                                                                                |
| `<C-w>s`      | Split horizontally                                                                              |
| `<C-h>`       | Move to left split                                                                              |
| `<C-j>`       | Move to lower split                                                                             |
| `<C-k>`       | Move to upper split                                                                             |
| `<C-l>`       | Move to right split                                                                             |
| `\`           | Enter search mode                                                                               |
| `:`           | Enter command mode                                                                              |
| `<space>sg`   | Search files by name (similar to [telescope](https://github.com/nvim-telescope/telescope.nvim)) |
| `<space>on`   | Open neovim in current directory (comes back to the app after closing neovim)                   |
| `m`           | Jump to item (similar to [flash](https://github.com/folke/flash.nvim))                          |
| `M`           | Jump to item and open it (see above)                                                            |

## Telescope

| Mappings  | Action             |
| --------- | ------------------ |
| `<C-n>`   | Next item          |
| `<C-p>`   | Previous item      |
| `<Enter>` | Open selected item |

# Commands

The only built-in command is the quit command, `q`, which closes the app.
One can use the terminal commands, such as `git status`, similarly in neovim, by pre-pending them with an exclamation mark: `!git status`, when in command mode.
