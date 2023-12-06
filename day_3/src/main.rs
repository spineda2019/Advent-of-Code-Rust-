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

            if !digit_already_found {
                digit_already_found = true;
                current_left = index;
            }

            if index == line.len() - 1 {
                let left: usize = current_left;
                let right: usize = index - 1;
                let number: String = current_number;

                nums_with_positions.push((left, right, number));

                current_left = 0;
                current_number = String::from("");

                digit_already_found = false;
                ready_to_push = false;
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

fn find_engine_sum(lines: &[String]) -> usize {
    let mut sum: usize = 0;
    // TODO: Check previous and later line for symbols (left + right too)
    // TODO: add to sum if valid
    for (index, each_line) in lines.iter().enumerate() {
        let line_info: Vec<(usize, usize, String)> = find_numbers(each_line);
        let line_length: usize = line_info.len();

        for each_tuple in line_info {
            println!(
                "Num: {:?}, Current Sum Before processing: {sum}",
                each_tuple
            );
            match index {
                0 => {
                    let prior_sum: usize = sum;

                    for below_index in each_tuple.0 - 1..=each_tuple.1 + 1 {
                        // TODO: don't look up
                        if index != lines.len() - 1
                            && lines[index + 1].chars().nth(below_index).unwrap_or('.') != '.'
                        {
                            println!(
                                " Below Chars: {:?}",
                                lines[index + 1].chars().nth(below_index).unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }
                    }

                    if prior_sum != sum {
                        continue;
                    }

                    if (each_tuple.0 != 0
                        && each_line.chars().nth(each_tuple.0 - 1).unwrap_or('.') != '.')
                        || (each_tuple.1 != line_length
                            && each_line.chars().nth(each_tuple.1 + 1).unwrap_or('.') != '.')
                    {
                        println!(
                            "Left Char: {}",
                            each_line.chars().nth(each_tuple.0 - 1).unwrap()
                        );
                        println!(
                            "Right Char: {}",
                            each_line.chars().nth(each_tuple.1 + 1).unwrap()
                        );

                        sum += match each_tuple.2.parse() {
                            Ok(x) => x,
                            Err(_e) => 0,
                        };
                    }
                }
                size if size == lines.len() - 1 => {
                    let prior_sum: usize = sum;

                    for above_index in each_tuple.0 - 1..=each_tuple.1 + 1 {
                        if index != 0
                            && lines[index - 1].chars().nth(above_index).unwrap_or('.') != '.'
                        {
                            println!(
                                " Above Chars: {:?}",
                                lines[index - 1].chars().nth(above_index).unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }
                    }

                    if prior_sum != sum {
                        continue;
                    }

                    if (each_tuple.0 != 0
                        && each_line.chars().nth(each_tuple.0 - 1).unwrap_or('.') != '.')
                        || (each_tuple.1 != line_length
                            && each_line.chars().nth(each_tuple.1 + 1).unwrap_or('.') != '.')
                    {
                        println!(
                            "Left Char: {}",
                            each_line.chars().nth(each_tuple.0 - 1).unwrap()
                        );
                        println!(
                            "Right Char: {}",
                            each_line.chars().nth(each_tuple.1 + 1).unwrap()
                        );
                        sum += match each_tuple.2.parse() {
                            Ok(x) => x,
                            Err(_e) => 0,
                        };
                    }
                }
                _middle_values => {
                    let prior_sum: usize = sum;

                    for above_and_below in each_tuple.0..=each_tuple.1 {
                        if index != lines.len() - 1
                            && lines[index + 1].chars().nth(above_and_below).unwrap_or('.') != '.'
                        {
                            println!(
                                " Below Chars: {:?}",
                                lines[index + 1].chars().nth(above_and_below).unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }

                        if prior_sum != sum {
                            break;
                        }

                        if index != 0
                            && lines[index - 1].chars().nth(above_and_below).unwrap_or('.') != '.'
                        {
                            println!(
                                " Above Chars: {:?}",
                                lines[index - 1].chars().nth(above_and_below).unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }
                    }

                    if prior_sum != sum {
                        continue;
                    }

                    // diags

                    if each_tuple.0 == 0 {
                        //just look right diagonal
                        if lines[index + 1]
                            .chars()
                            .nth(each_tuple.1 + each_tuple.2.len() - 1)
                            .unwrap()
                            != '.'
                        {
                            println!(
                                " Below Chars: {:?}",
                                lines[index + 1]
                                    .chars()
                                    .nth(each_tuple.1 + each_tuple.2.len() - 1)
                                    .unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }

                        if prior_sum != sum {
                            continue;
                        }

                        if lines[index - 1]
                            .chars()
                            .nth(each_tuple.1 + each_tuple.2.len() - 1)
                            .unwrap()
                            != '.'
                        {
                            println!(
                                " Above Chars: {:?}",
                                lines[index - 1]
                                    .chars()
                                    .nth(each_tuple.1 + each_tuple.2.len() - 1)
                                    .unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }

                        if prior_sum != sum {
                            continue;
                        }
                    }
                    if each_tuple.1 + each_tuple.2.len() - 1 == each_line.len() {
                        // just look left diagonal
                        if lines[index + 1]
                            .chars()
                            .nth(each_tuple.0 - 1)
                            .unwrap_or('.')
                            != '.'
                        {
                            println!(
                                " Below Chars: {:?}",
                                lines[index + 1].chars().nth(each_tuple.0 - 1).unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }

                        if prior_sum != sum {
                            continue;
                        }
                        if lines[index - 1]
                            .chars()
                            .nth(each_tuple.0 - 1)
                            .unwrap_or('.')
                            != '.'
                        {
                            println!(
                                " Above Chars: {:?}",
                                lines[index - 1].chars().nth(each_tuple.0 - 1).unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }

                        if prior_sum != sum {
                            continue;
                        }
                    } else {
                        // look at all diagonals

                        if lines[index + 1]
                            .chars()
                            .nth(each_tuple.0 - 1)
                            .unwrap_or('.')
                            != '.'
                        {
                            println!(
                                " Below Chars: {:?}",
                                lines[index + 1].chars().nth(each_tuple.0 - 1).unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }

                        if prior_sum != sum {
                            continue;
                        }

                        if lines[index + 1]
                            .chars()
                            .nth(each_tuple.1 + each_tuple.2.len() - 1)
                            .unwrap()
                            != '.'
                        {
                            println!(
                                " Below Chars: {:?}",
                                lines[index + 1]
                                    .chars()
                                    .nth(each_tuple.1 + each_tuple.2.len() - 1)
                                    .unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }

                        if prior_sum != sum {
                            continue;
                        }

                        // ABOVE
                        if lines[index - 1]
                            .chars()
                            .nth(each_tuple.0 - 1)
                            .unwrap_or('.')
                            != '.'
                        {
                            println!(
                                " Above Chars: {:?}",
                                lines[index - 1].chars().nth(each_tuple.0 - 1).unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }

                        if prior_sum != sum {
                            continue;
                        }

                        if lines[index - 1]
                            .chars()
                            .nth(each_tuple.1 + each_tuple.2.len() - 1)
                            .unwrap()
                            != '.'
                        {
                            println!(
                                " Above Chars: {:?}",
                                lines[index - 1]
                                    .chars()
                                    .nth(each_tuple.1 + each_tuple.2.len() - 1)
                                    .unwrap()
                            );
                            sum += match each_tuple.2.parse() {
                                Ok(x) => x,
                                Err(_e) => 0,
                            };
                        }

                        if prior_sum != sum {
                            continue;
                        }
                    }

                    if (each_tuple.0 != 0
                        && each_line.chars().nth(each_tuple.0 - 1).unwrap_or('.') != '.')
                        || (each_tuple.1 != line_length
                            && each_line
                                .chars()
                                .nth(each_tuple.1 + each_tuple.2.len() - 1)
                                .unwrap_or('.')
                                != '.')
                    {
                        println!("Left Char: {:?}", each_line.chars().nth(each_tuple.0 - 1));
                        println!("Right Char: {:?}", each_line.chars().nth(each_tuple.1 + 1));
                        sum += match each_tuple.2.parse() {
                            Ok(x) => x,
                            Err(_e) => 0,
                        };
                    }
                }
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
