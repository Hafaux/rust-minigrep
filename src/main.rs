use std::process;

fn main() {
    let args = minigrep::get_args().unwrap_or_else(|err| {
        eprintln!("Error while parsing arguments: {err}");

        process::exit(1)
    });

    if let Err(e) = minigrep::grep(args) {
        eprintln!("Application error {e}");

        process::exit(1)
    }
}
