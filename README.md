# anyrun org-todo
This plugin creates a todo with timestamp to your `inbox.org` file, inspired by [rofi-org-todo](https://github.com/wakatara/rofi-org-todo).
# How to use
- Adjust the path to your `inbox.org` file in `src/lib.rs`
- Run `cargo build --release` to build the binary
- Copy the binary to your anyrun plugins directory, e.g. `~/.config/anyrun/plugins`
# Configuration
You can configure the path to your `inbox.org` file by creating a `orgtodo.ron` file in the anyrun config directory.
```ron
// <Anyrun config dir>/orgtodo.ron
Config(
  inbox_file: "/path/to/inbox.org",
)
```
