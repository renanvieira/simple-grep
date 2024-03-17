mod argument_parser;

use std::{
    env,
    error::Error,
    io::{self, BufRead},
};

use crate::argument_parser::Arguments;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::parse(&args)?;

    let result = stdin
        .lines()
        .into_iter()
        .map(|s| s.unwrap())
        .enumerate()
        .find(|(_, s)| s.contains(&arguments.search_term));

    if let Some(r) = result {
        let mut text = r.1.clone();
        text = text.replace(
            &arguments.search_term,
            format!("{}{}{}", "\x1b[1m", &arguments.search_term, "\x1b[0m").as_ref(),
        );

        if arguments.show_line_number {
            println!("{}: {}", r.0, text);
        } else {
            println!("{}", text);
        }
    }

    Ok(())
}
