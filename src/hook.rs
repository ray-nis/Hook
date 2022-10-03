use std::{fs::File, io::{BufReader, BufRead}};
use crate::cli::Cli;
use regex::Regex;


pub fn hook(cli: Cli) {
    let pattern = Regex::new(&cli.pattern).unwrap();
    let source = File::open(&cli.source).expect("Invalid file.");
    let reader = BufReader::new(source);

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading file.");
        match pattern.find(&line) {
            Some(matched) => {
                let mut matched_word = String::from("\x1b[93m");
                matched_word.push_str(matched.as_str());
                matched_word.push_str("\x1b[0m");
                let colored_line = pattern.replace_all(&line, matched_word);
                println!("{}: {}", i+1, colored_line);
            },
            None => (),
        }
    }
}