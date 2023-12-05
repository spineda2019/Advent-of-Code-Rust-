use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_numbers(line: &str) -> Vec<(usize, usize, String)> {
    let mut nums_with_positions: Vec<(usize, usize, String)> = Vec::new();

    let mut ready_to_push: bool = false;
    let mut digit_already_found: bool = false;

    let mut current_number: String = String::from("");
    let mut current_left: usize = 0;

    for (index, each_character) in line.chars().enumerate() {
        if each_character.is_ascii_digit() {
            ready_to_push = true;
            current_number.push(each_character);

            if digit_already_found {
                // TODO
            } else {
                digit_already_found = true;
                current_left = index;
                // TODO
            }
        } else if ready_to_push {
            let left: usize = current_left;
            let right: usize = index - 1;
            let number: String = current_number;

            nums_with_positions.push((left, right, number));

            current_left = 0;
            current_number = String::from("");

            digit_already_found = false;
            ready_to_push = false;
        }
    }

    nums_with_positions
}

fn find_engine_sum(lines: &Vec<String>) -> usize {
    let sum: usize = 4;
    // TODO: Consume a line (dif func?) get indices of start and end of number
    // TODO: Check previous and later line for symbols (left + right too)
    // TODO: add to sum if valid
    for (index, each_line) in lines.iter().enumerate() {
        let line_info: Vec<(usize, usize, String)> = find_numbers(each_line);
        match &index {
            0 => {
                // TODO: don't look up
            }
            size if size == &line_info.len() => {
                // TODO: don't look down
            }
            middle_values => {
                // TODO
            }
        }
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
