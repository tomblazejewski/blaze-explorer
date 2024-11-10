A neovim-imspired file explorer.

Key bindings:

- j/k - move up and down
- backspace - move to parent directory
- enter - open folder/open file with default app
- dd - delete selected item
- r - rename selected item
- u - undo last action (delete/rename)
- Ctrl+r - redo last action
- n - next search result
- N - previous search result

Modes:

- normal (Esc to go back to it)
- search (/)
- command (:)

Splits:

- Ctrl+w, v - vertical split
- Ctrl+w, s - horizontal split
- Ctrl+h - move to left split
- Ctrl+l - move to right split
- Ctrl+k - move to upper split
- Ctrl+j - move to lower split

Plugins/custom actions:

- m - jump to item (similar to flash)
- M - jump and open selected item
- space+sg - search files by name (similar to telescope)
- space+on - open neovim in current directory (comes back to the app after closing neovim)
