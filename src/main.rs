use which_shell::get_shell;

fn main() {
    if let Some(sh) = get_shell() {
        println!("{}", sh)
    } else {
        println!("shell is not supported")
    }
}
