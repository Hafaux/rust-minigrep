mod tests;

use std::process;

fn main() {
    let params = minigrep::get_args().unwrap_or_else(|err| {
        eprintln!("Error while parsing arguments: {err}");

        process::exit(1)
    });

    match minigrep::run(params) {
        Ok(result) => {
            println!("{}", result.join("\n"))
        }
        Err(error) => {
            eprintln!("{error}");

            process::exit(1)
        }
    }
}
