# mmm - A tasty rust terminal file browser

This is currently a personal project to compliment my terminal workflow using a combination of neovim, ssh, and other cli tools. It is focused on fuzzy finding and keyboard first navigation and is only for navigation and opening neovim.

## `cd` on quit

To cd on quit you can add the following to your `$HOME/.bashrc`

```
m() {
    if command -v mmm &>/dev/null; then
        mmm
        if [ -f /tmp/mmm.path ]; then
            target_dir=$(< /tmp/mmm.path) # Read the file content into a variable
            cd "$target_dir" || echo "Failed to cd to $target_dir"
            rm -f /tmp/mmm.path # Delete the file
        fi
    else
        echo "Command 'mmm' not found."
    fi
}
```
Now by running `m` in the terminal it will cd after the program closes. Ensure you reload the terminal (close and open or source your bashrc) after adding this command to your `.bashrc`.

## Development

**MVP Features**

- [x] Create tui buffer seperate from main terminal screen
- [x] Use crossterm to make a tui interface
- [x] Design interface
- [x] Implement system calls for getting files, folders and file information asynchronously
- [ ] Display current, parent and child folders
- [x] Implement fzf folder changing
- [x] Implement nvim opener
- [ ] Implement previewer (maybe)
- [ ] Add cli options

**Ideas**

Navigation - I want to have the navigation such that you are typing a fuzzy search by default, then every space enters the directory with the best fuzzy match. Backspace clears any typed characters first then when the search string is empty it moves back a directory. Pressing space on a file rather than a directory will open nvim at either the git root of the file and open the file, or just open the file in nvim at the cwd.
