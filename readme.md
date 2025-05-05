A neovim-inspired, non-async file explorer, written purely in Rust.
Currently under development and unstable.

![App preview](docs/preview.png)

# App Mappings

## Normal mode

| Mappings      | Action                                                                        |
| ------------- | ----------------------------------------------------------------------------- |
| `j`           | Next item                                                                     |
| `k`           | Previous item                                                                 |
| `gg`          | First item                                                                    |
| `G`           | Last item                                                                     |
| `<Backspace>` | Move to parent directory                                                      |
| `<Enter>`     | Open selected item                                                            |
| `<C-o>`       | Go back in directory history                                                  |
| `<C-i>`       | Go forward in directory history                                               |
| `dd`          | Delete selected item                                                          |
| `yy`          | Copy selected item to clipboard                                               |
| `p`           | Paste from clipboard                                                          |
| `a`           | Add new item                                                                  |
| `r`           | Rename selected item                                                          |
| `R`           | Copy and rename selected item                                                 |
| `u`           | Undo last action (delete/rename)                                              |
| `<C-r>`       | Redo last action (delete/rename)                                              |
| `n`           | Next search result                                                            |
| `N`           | Previous search result                                                        |
| `<C-w>v`      | Split vertically                                                              |
| `<C-w>s`      | Split horizontally                                                            |
| `<C-h>`       | Move to left split                                                            |
| `<C-j>`       | Move to lower split                                                           |
| `<C-k>`       | Move to upper split                                                           |
| `<C-l>`       | Move to right split                                                           |
| `\`           | Enter search mode                                                             |
| `:`           | Enter command mode                                                            |
| `v`           | Enter visual mode                                                             |
| `<space>on`   | Open neovim in current directory (comes back to the app after closing neovim) |

## Visual mode

| Mappings | Action                                |
| -------- | ------------------------------------- |
| `j`      | Next item                             |
| `k`      | Previous item                         |
| `<Esc>`  | Exit visual mode and unmark all items |
| `e`      | Toggle mark selected item             |
| `d`      | Delete marked items                   |
| `y`      | Copy selected items to clipboard      |
| `p`      | Paste from clipboard                  |

### Git integration

| Mappings    | Action                                          |
| ----------- | ----------------------------------------------- |
| `<space>hc` | Git add and commit (waits to enter the message) |
| `<space>ht` | Show git status                                 |
| `<space>hP` | Push current branch to remote                   |
| `<space>hO` | Pull current branch from remote                 |

# Commands

The only built-in command is the quit command, `q`, which closes the app.
One can use the terminal commands, such as `git status`, similar to neovim, by pre-pending them with an exclamation mark: `!git status`, when in command mode.

# Plugins

The following plugins are available:

- [Telescope](https://github.com/tomblazejewski/blaze_telescope)
- [Flash](https://github.com/tomblazejewski/blaze_flash)

Given the core features of the library are not yet published as crate, integrating plugins must be done manually.
The app and plugins need to be arranged in the following structure for the libraries to be recognised:

```
.
├── blaze_explorer
└── blaze_plugins/
  ├── blaze_flash
  └── blaze_telescope.
```

Building of the projects needs to be done in a specific order:

1. Build the core library by running `cargo build --lib` inside of the `blaze_explorer` directory
2. Build the plugins by running `cargo build --lib` inside of the each subdirectory in the `blaze_plugins` directory
3. Build the app by running `cargo build` inside of the `blaze_explorer` directory

# To-do

- [ ] File manipulation abilities
  - [x] Delete
  - [ ] Cut
  - [x] Copy
  - [x] Paste
  - [ ] Enable motions of all of the above (see Keymap system)
  - [x] Delete backup files upon leaving the app
- [ ] Dir navigation
  - [x] Go up and down the history of a single ExplorerTable
  - [ ] Show directory history of the currenct ExplorerTable
  - [ ] Show currently open directories and jump to one of them
- [ ] Tooling
  - [ ] Show diffs between files in the same directory
- [ ] Git integration
  - [x] Show tracked/untracked/staged/unstaged/modified files
  - [ ] Implement shortcuts for commiting/pushing/checking out individual/ groups of files
  - [ ] Preview file changes with a shortcut
- [x] Plugin management
  - [x] Manage the non-core features through the Plugin trait
  - [x] Allow attaching certain plugins upon launching the app
  - [x] Allow defining custom keys to plugin actions (in code)
- [ ] Keymap system
  - [ ] Enable motions/multipliers for commands (e.g. 3dd)
  - [ ] Allow managing/adding keymaps through an accessible interface (lua script/toml file)
  - [x] Allow assigning keymaps to terminal commands
  - [ ] Allow searching for keymaps through Telescope
- [ ] Terminal commands
  - [ ] Allow autocompletion of commands
  - [ ] Show history of commands
- [x] Add a visual mode
