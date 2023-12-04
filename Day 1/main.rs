use std::io::{BufRead, BufReader};

fn find_first_digit(line: &String) -> &String {
    let first_digit = line.find(|c: char| c.is_digit(10));
    match first_digit {
        Some(index) => {println!("The first digit is at index {} and is {}", index, line[index])
        //
    }
        None => println!("The string does not contain any digits"),
    }
    return line;
}

fn find_last_digit(line: &String) -> &String {
    let first_digit = line.rfind(|c: char| c.is_digit(10));
    match first_digit {
        Some(index) => println!("The last digit is at index {} an is {}", index, line[index]),
        None => println!("The string does not contain any digits"),
    }
    return line;
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
                    find_first_digit(&line);
                    find_last_digit(&line);
                    result += 1;
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
