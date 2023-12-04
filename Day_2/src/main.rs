use std::io::BufRead;

fn find_first_digit(line: &String) -> &char {
    return &'c';
}

fn find_last_digit(line: &String) -> &char {
    return &'c';
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

        let file: std::fs::File = std::fs::File::open(file_name)?;
        let file_reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(file);

        for line in file_reader.lines() {
            match line {
                Ok(line) => {
                    let mut number: String = String::from("");
                    number.push(*find_first_digit(&line));
                    number.push(*find_last_digit(&line));
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
