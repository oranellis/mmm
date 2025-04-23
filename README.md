# mmm - A tasty rust terminal file browser

This is currently a personal project to compliment my terminal workflow using a combination of neovim, ssh, and other cli tools. It is focused on fuzzy finding and keyboard first navigation and is only for navigation right now. Feel free to try it out!

## Keybinds

Below is a list of keybinds to use the program
```
Esc        - quit the program
Backspace  - clear the filter or navigate to the parent folder
Space      - navigate into the top folder
Ctrl-h     - show hidden files
```

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

## Installation

Follow the instructions on the [releases](https://github.com/oranellis/mmm/releases) page. To download and install in one bash snippet on x86\_64 machines you can run the following,
```
wget -q --show-progress -O/tmp/mmm.tar.gz https://github.com/oranellis/mmm/releases/download/v0.3.2/mmm-linux-x86_64.tar.gz
tar -xzvf /tmp/mmm.tar.gz -C /tmp
mkdir -p ~/.local/bin
cp /tmp/mmm ~/.local/bin
```
Make sure to add `~/.local/bin` to your PATH variable if not present already.
