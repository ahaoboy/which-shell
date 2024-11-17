## which-shell

```shell
cargo binstall which-shell

which-shell

fish 3.7.1
```

## usage


```rust
use which_shell::which_shell;

fn main() {
    if let Some(sh) = which_shell() {
        println!("{}", sh)
    } else {
        println!("shell is not supported")
    }
}

```

## shell
- [x] Bash
- [x] Zsh
- [x] Fish
- [x] PowerShell
- [x] Pwsh
- [x] Cmd
- [x] Nu
- [x] Dash
- [x] Ksh
- [x] Tcsh
- [x] Csh