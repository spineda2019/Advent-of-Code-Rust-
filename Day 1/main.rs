use std::io::{BufRead, BufReader};

fn find_first_digit(line: &String) -> char {
    let answer: char;
    let first_digit = line.find(|c: char| c.is_digit(10));
    match first_digit {
        Some(index) => {
            let digit = line.chars().nth(index);
            match digit {
                Some(ch) => {
                    println!("The first digit is at index {} and is {}", index, ch);
                    answer = ch;
                }
                None => {
                    answer = '0';
                }
            }
        }
        None => {
            println!("The string does not contain any digits");
            answer = '0';
        }
    }
    return answer;
}

fn find_last_digit(line: &String) -> char {
    let answer: char;
    let first_digit = line.rfind(|c: char| c.is_digit(10));
    match first_digit {
        Some(index) => {
            let digit = line.chars().nth(index);
            match digit {
                Some(ch) => {
                    println!("The last digit is at index {} and is {}", index, ch);
                    answer = ch;
                }
                None => {
                    answer = '0';
                }
            }
        }
        None => {
            println!("The string does not contain any digits");
            answer = '0';
        }
    }
    return answer;
}

fn main() -> Result<(), std::io::Error> {
    let mut result: i64 = 0;

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Use: main.exe <TEXT_FILE>");
        std::process::exit(1);
    } else {
        let file_name: &String = &args[1];
        if !file_name.ends_with(".txt") {
            println!("Invalid file extension< use .txt");
            std::process::exit(1);
        }
        println!("Input {} found!", file_name);

        let file = std::fs::File::open(file_name)?;
        let file_reader = BufReader::new(file);

        for line in file_reader.lines() {
            match line {
                Ok(line) => {
                    
                    let mut number: String = String::from("");
                    number.push(find_first_digit(&line));
                    number.push(find_last_digit(&line));
                    let final_number: Result<i64, std::num::ParseIntError> = number.parse();
                    match final_number {
                        Ok(final_number) => {
                            println!("{}", final_number);
                            result += final_number;
                        }
                        Err(_e) => {
                            continue;
                        }
                    }
                }
                Err(_e) => {
                    continue;
                }
            }
        }

        println!("Result: {}", result);

        Ok(())
    }
}
