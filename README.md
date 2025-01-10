# MMM - A tasty and feature rich rust terminal file browser

This is currently a personal project to compliment my terminal workflow using a combination of neovim, ssh, and other cli tools. It builds on the design philosophies of [nnn](https://github.com/jarun/nnn) while adding some vim-like modes and bindings to speed up operation.

## Development

**MVP Features**

- [x] Create tui buffer seperate from main terminal screen
- [x] Use crossterm to make a tui interface
- [ ] Design interface
- [ ] Implement system calls for getting files, folders and file information asynchronously
- [ ] Display current, parent and child folders
- [ ] Implement fzf folder changing
- [ ] Implement nvim opener
- [ ] Implement previewer (maybe)
- [ ] Add cli options

**Ideas**

Navigation - I want to have the navigation such that you are typing a fuzzy search by default, then every space enters the directory with the best fuzzy match. Backspace clears any typed characters first then when the search string is empty it moves back a directory. Pressing space on a file rather than a directory will open nvim at either the git root of the file and open the file, or just open the file in nvim at the cwd.
