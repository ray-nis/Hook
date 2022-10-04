use std::{fs::File, io::{BufReader, BufRead}};
use crate::cli::Cli;
use regex::Regex;
use colored::*;


pub fn hook(cli: Cli) {
    let mut pat: String = cli.pattern;
    if cli.whole_word {
        let mut whole_word_regex = String::from("\\b");
        whole_word_regex.push_str(&pat);
        whole_word_regex.push_str("\\b");
        pat = whole_word_regex;
    }
    if cli.ignore_case {
        let mut ignore_case_regex = String::from("(?i)");
        ignore_case_regex.push_str(&pat);
        pat = ignore_case_regex;
    }
    let pattern = Regex::new(&pat).unwrap();
    let source = File::open(&cli.source).expect("Invalid file.");
    let reader = BufReader::new(source);

    let mut count: usize = 0;

    for (i, line) in reader.lines().enumerate() {
        let mut line = line.expect("Error reading file.");
        match pattern.is_match(&line) {
            true => {
                count += 1;
                if cli.count == true {
                    continue;
                }
                if cli.number_line {
                    print!("{}: ", i);
                }
                if cli.pattern_highlighter {
                    print_line_pattern_highlighted(&pattern, &mut line);
                } else {
                    println!("{}", &line);
                }
            },
            false => (),
        }
    }

    if cli.count == true {
        println!("{}", count);
    }
}

fn print_line_pattern_highlighted(pattern: &Regex, line: &mut String) {
    match pattern.find(&line) {
        Some(mat) => {
            print!("{}", &line[0..mat.start()]);
            print!("{}", &line[mat.start()..mat.end()].cyan());
            print_line_pattern_highlighted(pattern, &mut line[mat.end()..].to_string());
        },
        None => println!("{}", line),
    }
}