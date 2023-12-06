use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_in_winners(number_literal: &str, winners: &str) -> bool {
    for number in winners.split_whitespace() {
        if number_literal.parse().unwrap_or(-1) == number.parse().unwrap_or(-2) {
            println!("{number_literal} is in {winners}!");
            return true;
        }
    }
    println!("{number_literal} is NOT in {winners}...");
    false
}

fn calculate_points(line: &str, sum: &mut isize) {
    let winners: &str;
    let numbers: &str;

    (winners, numbers) = (
        line.split(" | ").collect::<Vec<&str>>()[0]
            .split(": ")
            .collect::<Vec<&str>>()[1],
        line.split(" | ").collect::<Vec<&str>>()[1],
    );

    let mut matches: u32 = 0;

    for number in winners.split_whitespace() {
        if is_in_winners(number, numbers) {
            matches += 1;
        }
    }

    *sum += isize::pow(2, matches - 1);
}

fn main() -> Result<(), std::io::Error> {
    let mut file_name: String = "".to_string();

    {
        let mut parser: ArgumentParser<'_> = ArgumentParser::new();
        parser.set_description("Parse a text input for Day 2 of Advent of Code");
        parser
            .refer(&mut file_name)
            .add_option(&["-f", "--file"], Store, "Text File Name");
        parser.parse_args_or_exit();
    }

    let file_name: String = file_name;

    if file_name.is_empty() {
        println!("Use: main.exe <TEXT_FILE>");
        std::process::exit(1);
    }

    if !file_name.ends_with(".txt") {
        println!("<TEXT_FILE> must end with .txt!");
        std::process::exit(1);
    }

    println!("File to read: {}", file_name);

    let file: File = File::open(file_name)?;
    let file_reader: BufReader<&File> = BufReader::new(&file);

    let mut sum: isize = 0;

    for line in file_reader.lines() {
        calculate_points(&line.unwrap_or(String::from("")), &mut sum);
    }

    println!("Scratch Sum: {sum}");

    Ok(())
}
