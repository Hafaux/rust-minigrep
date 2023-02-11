#[cfg(test)]
mod tests {
    use minigrep::*;

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
