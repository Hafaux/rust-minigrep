use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Params {
    path: String,
    query: String,
}

impl Params {
    fn build(args: &[String]) -> Result<Params, &'static str> {
        if args.len() != 3 {
            return Err("Not enough args, usage: minigrep filename.txt <search_string>");
        }

        Ok(Params {
            path: args[1].clone(),
            query: args[2].clone(),
        })
    }
}

fn grep(params: Params) -> Result<(), Box<dyn Error>> {
    let match_line = |line: &&str| line.to_lowercase().contains(params.query.as_str());

    let matching_lines = fs::read_to_string(params.path)?
        .split("\n")
        .filter(match_line)
        .collect::<Vec<&str>>()
        .join("\n");

    print!("\n{}", matching_lines);

    Ok(())
}

fn get_args() -> Result<Params, &'static str> {
    let args: Vec<String> = env::args().collect();

    Params::build(&args)
}

fn main() {
    let args = get_args().unwrap_or_else(|err| {
        eprintln!("Error while parsing arguments: {err}");

        process::exit(1)
    });

    if let Err(e) = grep(args) {
        eprintln!("Application error {e}");

        process::exit(1)
    }
}
