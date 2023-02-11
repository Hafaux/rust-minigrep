use std::env;
use std::error::Error;
use std::fs;

pub struct Params {
    pub path: String,
    pub query: String,
}

impl Params {
    pub fn build(args: &[String]) -> Result<Params, &'static str> {
        if args.len() != 3 {
            return Err("Not enough args, usage: minigrep filename.txt <search_string>");
        }

        Ok(Params {
            path: args[1].clone(),
            query: args[2].clone(),
        })
    }
}

pub fn grep(params: Params) -> Result<(), Box<dyn Error>> {
    let match_line = |line: &&str| line.to_lowercase().contains(params.query.as_str());

    let matching_lines = fs::read_to_string(params.path)?
        .split("\n")
        .filter(match_line)
        .collect::<Vec<&str>>()
        .join("\n");

    print!("\n{}", matching_lines);

    Ok(())
}

pub fn get_args() -> Result<Params, &'static str> {
    let args: Vec<String> = env::args().collect();

    Params::build(&args)
}
