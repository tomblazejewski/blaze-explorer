A neovim-imspired file explorer.

Key bindings:

- j/k - move up and down
- backspace - move to parent directory
- enter - open folder/open file with default app
- dd - delete selected item
- r - rename selected item
- u - undo last action (delete/rename)
- <C-r> - redo last action
- n - next search result
- N - previous search result

Modes:

- normal (Esc to go back to it)
- search (/)
- command (:)

Splits:

- <C-w>v - vertical split
- <C-w>s - horizontal split
- <C-h> - move to left split
- <C-l> - move to right split
- <C-k> - move to upper split
- <C-j> - move to lower split

Plugins/custom actions:

- m - jump to item (similar to flash)
- M - jump and open selected item
- <space>sg - search files by name (similar to telescope)
- <space>on - open neovim in current directory (comes back to the app after closing neovim)
