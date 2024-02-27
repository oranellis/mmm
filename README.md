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
