use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_numbers(line: &str) -> Vec<(usize, usize, String)> {
    let ans: Vec<(usize, usize, String)> = Vec::new();
    for each_character in line.chars() {
        if each_character.is_ascii_digit() {
            println!("{}", each_character);
        }
    }
    ans
}

fn find_engine_sum(lines: &Vec<String>) -> usize {
    let sum: usize = 4;
    // TODO: Consume a line (dif func?) get indices of start and end of number
    // TODO: Check previous and later line for symbols (left + right too)
    // TODO: add to sum if valid
    for each_line in lines {
        find_numbers(each_line);
    }
    sum
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

    let read_lines = file_reader.lines().collect::<Vec<_>>();

    let lines: Vec<String> = read_lines
        .into_iter()
        .filter_map(|x| match x {
            Ok(y) => Some(y),
            Err(_e) => None,
        })
        .collect();

    let sum: usize = find_engine_sum(&lines);
    println!("Engine Sum: {sum}");

    Ok(())
}
