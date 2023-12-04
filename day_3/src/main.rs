use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_valid_id(line: &String) -> isize {
    let max_red: isize = 12;
    let max_green: isize = 13;
    let max_blue: isize = 14;

    let info: Vec<&str> = line
        .split(|delim| delim == ',' || delim == ';' || delim == ':')
        .collect();
    let id: Result<_, _> = info[0].split(' ').collect::<Vec<&str>>()[1].parse::<isize>();
    let id_number: isize = match id {
        Ok(x) => x,
        Err(_e) => 0,
    };

    let mut result: isize = -1;

    for (i, element) in info.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let color = element.split(' ').collect::<Vec<&str>>()[2];
        let number = element.split(' ').collect::<Vec<&str>>()[1];

        if color == "red" {
            if number.parse::<isize>().unwrap() > max_red {
                result = 0;
            }
        } else if color == "green" {
            if number.parse::<isize>().unwrap() > max_green {
                result = 0;
            }
        } else
        /* blue */
        {
            if number.parse::<isize>().unwrap() > max_blue {
                result = 0;
            }
        }

        if result != 0 {
            result = id_number;
        }
    }

    return result;
}

fn get_power(line: &String) -> isize {
    let info: Vec<&str> = line
        .split(|delim| delim == ',' || delim == ';' || delim == ':')
        .collect();

    let mut max_red: isize = 0;
    let mut max_green: isize = 0;
    let mut max_blue: isize = 0;

    for (i, element) in info.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let color: &str = element.split(' ').collect::<Vec<&str>>()[2];
        let number: &str = element.split(' ').collect::<Vec<&str>>()[1];

        if color == "red" {
            if number.parse::<isize>().unwrap() > max_red {
                max_red = number.parse::<isize>().unwrap();
            }
        } else if color == "green" {
            if number.parse::<isize>().unwrap() > max_green {
                max_green = number.parse::<isize>().unwrap();
            }
        } else
        /* blue */
        {
            if number.parse::<isize>().unwrap() > max_blue {
                max_blue = number.parse::<isize>().unwrap();
            }
        }
    }

    return max_blue * max_green * max_red;
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
    let file_reader: BufReader<&File> = BufReader::new(&file);

    let mut result: isize = 0;
    let mut power: isize = 0;

    for line in file_reader.lines() {
        result += match &line {
            Ok(x) => get_valid_id(&x),
            Err(_e) => 0,
        };

        power += match &line {
            Ok(x) => get_power(&x),
            Err(_e) => 0,
        };
    }

    println!("Answer: {}", result);
    println!("Power: {}", power);

    return Ok(());
}
