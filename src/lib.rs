use std::env;
use std::error::Error;
use std::fs;

pub struct Params {
    pub path: String,
    pub query: String,
    pub ignore_case: bool,
}

impl Params {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Params, &'static str> {
        args.next();

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("No path parameter provided"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No search string provided"),
        };

        let ignore_case_var = env::var("IGNORE_CASE").unwrap_or("false".to_owned());
        let ignore_case = matches!(ignore_case_var.as_str(), "1" | "true" | "t");

        Ok(Params {
            path,
            query,
            ignore_case,
        })
    }
}

fn line_matches(line: &str, query: &str, ignore_case: bool) -> bool {
    if ignore_case {
        line.to_lowercase().contains(query.to_lowercase().as_str())
    } else {
        line.to_owned().contains(query)
    }
}

pub fn find_string(
    contents: &str,
    query: &str,
    ignore_case: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let result = contents
        .lines()
        .filter(|line: &&str| line_matches(&line, &query, ignore_case))
        .map(|str| str.to_owned())
        .collect();

    Ok(result)
}

pub fn run(params: Params) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(params.path)?;
    let matching_lines = find_string(&contents, &params.query, params.ignore_case)?;

    Ok(matching_lines)
}

pub fn get_args() -> Result<Params, &'static str> {
    Params::build(env::args())
}
