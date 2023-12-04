use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_first_digit_word(line: &String) -> (i64, &str) {
    let numbers: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut answer: i64 = i64::MAX;
    let mut spelled_word: &str = "";

    for num in &numbers {
        if line.contains(num) {
            let index: i64 = line.find(num).unwrap() as i64;
            if index < answer {
                answer = index;
                spelled_word = num;
            }
        }
    }

    return (answer, spelled_word);
}

fn find_last_digit_word(line: &String) -> (i64, &str) {
    let numbers: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut answer: i64 = -1;
    let mut spelled_word: &str = "";

    for num in &numbers {
        if line.contains(num) {
            let index: i64 = line.rfind(num).unwrap() as i64;
            if index > answer {
                answer = index;
                spelled_word = num;
            }
        }
    }

    return (answer, spelled_word);
}

fn find_first_and_last(line: &String) -> i64 {
    let first_digit_option: Option<usize> = line.find(|c: char| c.is_digit(10));
    let first_digit_index: i64 = match first_digit_option {
        Some(index) => index as i64,
        None => -1,
    };

    let last_digit_option: Option<usize> = line.rfind(|c: char| c.is_digit(10));
    let last_digit_index: i64 = match last_digit_option {
        Some(index) => index as i64,
        None => -1,
    };

    let first_word_value: &str;
    let last_word_value: &str;
    let first_word_index: i64;
    let last_word_index: i64;

    (first_word_index, first_word_value) = find_first_digit_word(line);
    (last_word_index, last_word_value) = find_last_digit_word(line);

    let first_char: char;
    let second_char: char;

    if first_digit_index < first_word_index {
        first_char = line.chars().nth(first_digit_index as usize).unwrap();
    } else {
        first_char = match first_word_value {
            "one" => '1',
            "two" => '2',
            "three" => '3',
            "four" => '4',
            "five" => '5',
            "six" => '6',
            "seven" => '7',
            "eight" => '8',
            "nine" => '9',
            &_ => '0'
        };
    }

    if last_digit_index > last_word_index {
        second_char = line.chars().nth(last_digit_index as usize).unwrap();
    } else {
        second_char = match last_word_value {
            "one" => '1',
            "two" => '2',
            "three" => '3',
            "four" => '4',
            "five" => '5',
            "six" => '6',
            "seven" => '7',
            "eight" => '8',
            "nine" => '9',
            &_ => '0'
        };
    }

    return format!("{}{}", first_char, second_char).parse().unwrap();
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

    if file_name.len() == 0 {
        println!("Use: main.exe <TEXT_FILE>");
        std::process::exit(1);
    }

    if !file_name.ends_with(".txt") {
        println!("<TEXT_FILE> must end with .txt!");
        std::process::exit(1);
    }

    println!("File to read: {}", file_name);

    let file: File = File::open(file_name)?;
    let file_reader = BufReader::new(&file);

    let mut result: i64 = 0;

    for line in file_reader.lines() {
        let combined: i64 = match line {
            Ok(x) => find_first_and_last(&x),
            Err(_e) => 0,
        };

        result += combined;
    }

    println!("Answer: {}", result);

    return Ok(());
}
