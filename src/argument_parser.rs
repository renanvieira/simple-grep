use std::error::Error;

#[derive(Debug)]
pub struct Arguments {
    pub show_line_number: bool,
    pub search_term: String,
}

impl Arguments {
    pub fn new(show_line_number: bool, search_term: String) -> Self {
        Self {
            show_line_number,
            search_term,
        }
    }

    pub fn parse(args: &[String]) -> Result<Arguments, Box<dyn Error>> {
        let mut local_args = args.to_owned();
        let line_number_arg = local_args.iter().position(|p| p == "-n");

        let show_line_number = match line_number_arg {
            Some(ln_arg) => {
                local_args.remove(ln_arg);
                true
            }
            None => false,
        };

        let needle = local_args.last();
        match needle {
            Some(n) => Ok(Arguments::new(show_line_number, n.to_owned())),
            None => Err("Search term missing...".into()),
        }
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
