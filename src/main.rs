mod argument_parser;

use std::{
    env,
    error::Error,
    io::{self, Read}, process::exit,
};

use crate::argument_parser::Arguments;

const SHELL_STYLE_BOLD_START: &'static str = "\x1b[1m";
const SHELL_STYLE_BOLD_END: &'static str = "\x1b[0m";
const SHELL_STYLE_BOLD_RED_START : &'static str = "\x1b[1;31m";

fn search(arguments: Arguments, input: &str) -> Result<String, Box<dyn Error>> {
    let result = input
        .lines()
        .into_iter()
        .enumerate()
        .find(|(_, s)| s.contains(&arguments.search_term));

    if let Some(r) = result {
        let bold_text = r.1.replace(
            &arguments.search_term,
            format!(
                "{}{}{}",
                SHELL_STYLE_BOLD_START, &arguments.search_term, SHELL_STYLE_BOLD_END
            )
            .as_ref(),
        );

        if arguments.show_line_number {
            return Ok(format!("{}: {}", r.0, bold_text));
        } else {
            return Ok(format!("{}", bold_text));
        }
    }

    Ok(String::default())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input: String = String::default();
    let args: Vec<String> = env::args().into_iter().skip(1).collect();
    let arguments_result = Arguments::parse(&args);

    let arguments = match arguments_result {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}ERROR: {}{}", SHELL_STYLE_BOLD_RED_START, err, SHELL_STYLE_BOLD_END);
            exit(1);
        }
    };

    {
        let mut stdin = io::stdin().lock();
        let _ = stdin.read_to_string(&mut input);
    }

    let output = search(arguments, input.as_ref())?;

    println!("{}", output);


    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{argument_parser::Arguments, search, SHELL_STYLE_BOLD_END, SHELL_STYLE_BOLD_START};

    #[test]
    fn test_find_word() -> Result<(), Box<dyn Error>> {
        let input = "test\ntext\n1\n2\n3";
        let needle = "text".to_owned();
        let args = Arguments::new(false, needle.clone());

        let output = search(args, input.as_ref())?;

        assert!(output.contains(&needle));
        assert!(output.contains(&SHELL_STYLE_BOLD_START));
        assert!(output.contains(&SHELL_STYLE_BOLD_END));

        Ok(())
    }
}
