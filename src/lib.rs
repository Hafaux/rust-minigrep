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

pub fn find_string(contents: String, query: String) -> Result<Vec<String>, Box<dyn Error>> {
    let result = contents
        .lines()
        .filter(|line: &&str| line.to_lowercase().contains(query.as_str()))
        .map(|str| str.to_owned())
        .collect();

    Ok(result)
}

pub fn run(params: Params) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(params.path)?;
    let matching_lines = find_string(contents, params.query)?;

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
    fn result_one() {
        let query = String::from("frog");
        let contents = String::from(
            "\
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!",
        );

        let result = find_string(contents, query).unwrap();

        assert_eq!(vec!["How public, like a frog",], result);
    }

    #[test]
    fn result_two() {
        let query = String::from("body");
        let contents = String::from(
            "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.",
        );

        let result = find_string(contents, query).unwrap();

        assert_eq!(
            vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
            result
        );
    }
}
