use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_number_of_cards(matches: &[u32]) -> u32 {
    let mut card_array: Vec<u32> = std::iter::repeat(1).take(matches.len()).collect();

    for (index, number) in matches.iter().enumerate() {
        for _repitions in 0..card_array[index] {
            for i in index + 1..=index + *number as usize {
                card_array[i] += 1;
            }
        }
    }

    println!("Card Array: {:?}", card_array);
    let cards: u32 = card_array.iter().sum();
    cards
}

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

fn calculate_points(line: &str, sum: &mut isize) -> u32 {
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

    matches
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
    let mut vec_matches: Vec<u32> = Vec::new();

    for line in file_reader.lines() {
        let matches = calculate_points(&line.unwrap_or(String::from("")), &mut sum);
        vec_matches.push(matches);
    }

    let cards: u32 = calculate_number_of_cards(&vec_matches);

    println!("\nScratch Sum: {sum}");
    println!("Card Sum: {cards}");
    println!("{:?}", vec_matches);

    Ok(())
}
