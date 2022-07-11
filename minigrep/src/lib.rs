use std::env;
use std::error::Error;
use std::fs;

pub fn execute(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    if config.case_sensitive {
        for matche_line in search(&config.pattern, &content) {
            println!("{}", matche_line);
        }
    } else {
        for matche_line in search_insensitive(&config.pattern, &content) {
            println!("{}", matche_line);
        }
    }
    Ok(())
}

fn search<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

fn search_insensitive<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern.to_lowercase()))
        .collect()
}

pub struct Config {
    pattern: String,
    file_path: String,
    case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("should have 2 arguements".to_string());
        }
        let pattern = args[1].clone();
        let file_path = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            pattern,
            file_path,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pattern_in_content() {
        let pattern = "This";
        let content = "\
        This is an apple.
        That is a book.
        ";
        assert_eq!(vec!["This is an apple."], search(pattern, content));
    }

    #[test]
    fn case_insensitive() {
        let pattern = "this";
        let content = "\
        This is an apple.
        That is a book.
        ";
        assert_eq!(
            vec!["This is an apple."],
            search_insensitive(pattern, content)
        );
    }
}
