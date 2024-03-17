use std::error::Error;

#[derive(Debug)]
pub struct Arguments {
    pub show_line_number: bool,
    pub search_term: String,
}

impl Arguments {
    pub fn new(show_line_number: bool, filename: String) -> Self {
        Self {
            show_line_number,
            search_term: filename,
        }
    }

    pub fn parse(args: &Vec<String>) -> Result<Arguments, Box<dyn Error>> {
        let mut local_args = args.clone();
        let line_number_arg = local_args.iter().position(|p| p == "-n");

        let show_line_number = match line_number_arg {
            Some(ln_arg) => {
                local_args.remove(ln_arg);
                true
            }
            None => false,
        };

        let needle = local_args.last().expect("Search string missing...");

        Ok(Arguments::new(show_line_number, needle.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::Arguments;

    #[test]
    fn test_argument_parse_successfully() -> Result<(), Box<dyn Error>> {
        let args = vec![String::from("-n"), String::from("file.txt")];

        let result = Arguments::parse(&args)?;

        assert!(result.show_line_number);
        assert_eq!(result.search_term, "file.txt");

        Ok(())
    }

    #[test]
    fn test_argument_parse_without_filename() -> Result<(), Box<dyn Error>> {
        let args = vec![String::from("-n")];

        let result = Arguments::parse(&args);

        assert!(result.is_err());

        Ok(())
    }
}
