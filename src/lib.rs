use std::env;
use std::error::Error;
use std::fs;

pub struct Params {
    pub path: String,
    pub query: String,
    pub ignore_case: bool,
}

impl Params {
    pub fn build(args: &[String]) -> Result<Params, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args, usage: minigrep filename.txt <search_string>");
        }

        let ignore_case_var = env::var("IGNORE_CASE").unwrap_or("false".to_owned());
        let ignore_case = matches!(ignore_case_var.as_str(), "1" | "true" | "t");

        Ok(Params {
            path: args[1].clone(),
            query: args[2].clone(),
            ignore_case,
        })
    }
}

fn line_matches(line: &str, query: &String, ignore_case: bool) -> bool {
    if ignore_case {
        line.to_lowercase().contains(query.to_lowercase().as_str())
    } else {
        line.to_owned().contains(query)
    }
}

pub fn find_string(
    contents: String,
    query: String,
    ignore_case: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let result = contents
        .lines()
        .filter(|line: &&str| line_matches(line, &query, ignore_case))
        .map(|str| str.to_owned())
        .collect();

    Ok(result)
}

pub fn run(params: Params) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(params.path)?;
    let matching_lines = find_string(contents, params.query, params.ignore_case)?;

    Ok(matching_lines)
}

pub fn get_args() -> Result<Params, &'static str> {
    let args: Vec<String> = env::args().collect();

    Params::build(&args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_one() {
        let query = String::from("frog");
        let contents = String::from(
            "\
How dreary to be somebody!
How public, like a Frog
To tell your name the livelong day
To an admiring bog!",
        );

        let result = find_string(contents, query, true).unwrap();

        assert_eq!(vec!["How public, like a Frog"], result);
    }

    #[test]
    fn search_two() {
        let query = String::from("body");
        let contents = String::from(
            "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.",
        );

        let result = find_string(contents, query, false).unwrap();

        assert_eq!(
            vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
            result
        );
    }

    #[test]
    fn search_insensitive() {
        let query = String::from("FrOG");
        let contents = String::from(
            "\
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!",
        );

        let result = find_string(contents, query, true).unwrap();

        assert_eq!(vec!["How public, like a frog"], result);
    }

    #[test]
    fn search_sensitive() {
        let query = String::from("to");
        let contents = String::from(
            "\
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!",
        );

        let result = find_string(contents, query, false).unwrap();

        assert_eq!(vec!["How dreary to be somebody!"], result);
    }
}
