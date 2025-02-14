use which_shell::which_shell;

fn main() {
    if let Some(sh) = which_shell() {
        println!("{:?}", sh)
    } else {
        println!("shell is not supported")
    }
}
