use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct EngineMatrix {
    row_count: usize,
    column_count: usize,
    data: Vec<Vec<char>>,
}

impl EngineMatrix {
    fn new(data_file: &File) -> Option<EngineMatrix> {
        let file_reader: BufReader<&File> = BufReader::new(data_file);
        let mut matrix_data: Vec<Vec<char>> = Vec::new();

        for line in file_reader.lines() {
            let array: Vec<char> = match line {
                Ok(x) => x.chars().collect::<Vec<char>>(),
                Err(_) => return None,
            };
            matrix_data.push(array);
        }

        Some(EngineMatrix {
            row_count: matrix_data.len(),
            column_count: matrix_data[0].len(),
            data: matrix_data,
        })
    }

    fn engine_sum(&self) -> usize {
        let mut sum: usize = 0;
        let mut current_number: Vec<char> = Vec::new();

        for (row, line) in self.data.iter().enumerate() {
            for (column, symbol) in line.iter().enumerate() {
                if symbol.is_ascii_digit() {
                    current_number.push(*symbol);
                    if column == self.column_count - 1 {
                        // check now, at EOL
                        /*
                        ...878|
                             ^
                             We are here
                         */
                        if line[column - current_number.len()] != '.' {
                            let sum_result: Result<usize, _> = current_number
                                .clone()
                                .into_iter()
                                .collect::<String>()
                                .parse();
                            sum += sum_result.unwrap_or(0);
                            println!("Added {:?} on line {row}", current_number);
                        } else {
                            // check above and below
                            match row {
                                0 => {
                                    for i in column - current_number.len()..=column {
                                        // no up
                                        if self.data[row + 1][i] != '.' {
                                            let sum_result: Result<usize, _> = current_number
                                                .clone()
                                                .into_iter()
                                                .collect::<String>()
                                                .parse();
                                            sum += sum_result.unwrap_or(0);
                                            println!("Added {:?} on line {row}", current_number);
                                            break;
                                        }
                                    }
                                }
                                x if x == self.row_count - 1 => {
                                    for i in column - current_number.len()..=column {
                                        // no down
                                        if self.data[row - 1][i] != '.' {
                                            let sum_result: Result<usize, _> = current_number
                                                .clone()
                                                .into_iter()
                                                .collect::<String>()
                                                .parse();
                                            sum += sum_result.unwrap_or(0);
                                            println!("Added {:?} on line {row}", current_number);
                                            break;
                                        }
                                    }
                                }
                                _ => {
                                    for i in column - current_number.len()..=column {
                                        // both
                                        if self.data[row + 1][i] != '.' {
                                            let sum_result: Result<usize, _> = current_number
                                                .clone()
                                                .into_iter()
                                                .collect::<String>()
                                                .parse();
                                            sum += sum_result.unwrap_or(0);
                                            println!("Added {:?} on line {row}", current_number);
                                            break;
                                        }
                                        if self.data[row - 1][i] != '.' {
                                            let sum_result: Result<usize, _> = current_number
                                                .clone()
                                                .into_iter()
                                                .collect::<String>()
                                                .parse();
                                            sum += sum_result.unwrap_or(0);
                                            println!("Added {:?} on line {row}", current_number);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        current_number.clear();
                    }
                } else if !current_number.is_empty() {
                    // begin analysis
                    /*
                    ...878...
                          ^
                          We are here
                     */

                    if *symbol != '.' {
                        let sum_result: Result<usize, _> = current_number
                            .clone()
                            .into_iter()
                            .collect::<String>()
                            .parse();
                        sum += sum_result.unwrap_or(0);
                        println!("Added {:?} on line {row}", current_number);
                        current_number.clear();
                        continue;
                    }

                    // look left IF POSSIBLE
                    if column - current_number.len() != 0 {
                        // look left
                        if self.data[row][column - current_number.len() - 1] != '.' {
                            let sum_result: Result<usize, _> = current_number
                                .clone()
                                .into_iter()
                                .collect::<String>()
                                .parse();
                            sum += sum_result.unwrap_or(0);
                            println!("Added {:?} on line {row}", current_number);
                            current_number.clear();
                            continue;
                        }
                    }

                    // look up and down IF POSSIBLE
                    match (row, column - current_number.len() == 0) {
                        (0, true) => {
                            for i in column - current_number.len()..=column {
                                // no up
                                if self.data[row + 1][i] != '.' {
                                    let sum_result: Result<usize, _> = current_number
                                        .clone()
                                        .into_iter()
                                        .collect::<String>()
                                        .parse();
                                    sum += sum_result.unwrap_or(0);
                                    println!("Added {:?} on line {row}", current_number);
                                    break;
                                }
                            }
                        }
                        (x, true) if x == self.row_count - 1 => {
                            for i in column - current_number.len()..=column {
                                // no down
                                if self.data[row - 1][i] != '.' {
                                    let sum_result: Result<usize, _> = current_number
                                        .clone()
                                        .into_iter()
                                        .collect::<String>()
                                        .parse();
                                    sum += sum_result.unwrap_or(0);
                                    println!("Added {:?} on line {row}", current_number);
                                    break;
                                }
                            }
                        }
                        (_, true) => {
                            for i in column - current_number.len()..=column {
                                // both
                                if self.data[row + 1][i] != '.' {
                                    let sum_result: Result<usize, _> = current_number
                                        .clone()
                                        .into_iter()
                                        .collect::<String>()
                                        .parse();
                                    sum += sum_result.unwrap_or(0);
                                    println!("Added {:?} on line {row}", current_number);
                                    break;
                                }
                                if self.data[row - 1][i] != '.' {
                                    let sum_result: Result<usize, _> = current_number
                                        .clone()
                                        .into_iter()
                                        .collect::<String>()
                                        .parse();
                                    sum += sum_result.unwrap_or(0);
                                    println!("Added {:?} on line {row}", current_number);
                                    break;
                                }
                            }
                        }
                        /* ------------------------------------------------- */
                        (0, _) => {
                            for i in column - current_number.len() - 1..=column {
                                // no up
                                if self.data[row + 1][i] != '.' {
                                    let sum_result: Result<usize, _> = current_number
                                        .clone()
                                        .into_iter()
                                        .collect::<String>()
                                        .parse();
                                    sum += sum_result.unwrap_or(0);
                                    println!("Added {:?} on line {row}", current_number);
                                    break;
                                }
                            }
                        }
                        (y, _) if y == self.row_count - 1 => {
                            for i in column - current_number.len() - 1..=column {
                                // no down
                                if self.data[row - 1][i] != '.' {
                                    let sum_result: Result<usize, _> = current_number
                                        .clone()
                                        .into_iter()
                                        .collect::<String>()
                                        .parse();
                                    sum += sum_result.unwrap_or(0);
                                    println!("Added {:?} on line {row}", current_number);
                                    break;
                                }
                            }
                        }
                        (_, _) => {
                            for i in column - current_number.len() - 1..=column {
                                // both
                                if self.data[row + 1][i] != '.' {
                                    let sum_result: Result<usize, _> = current_number
                                        .clone()
                                        .into_iter()
                                        .collect::<String>()
                                        .parse();
                                    sum += sum_result.unwrap_or(0);
                                    println!("Added {:?} on line {row}", current_number);
                                    break;
                                }
                                if self.data[row - 1][i] != '.' {
                                    let sum_result: Result<usize, _> = current_number
                                        .clone()
                                        .into_iter()
                                        .collect::<String>()
                                        .parse();
                                    sum += sum_result.unwrap_or(0);
                                    println!("Added {:?} on line {row}", current_number);
                                    break;
                                }
                            }
                        }
                    }
                    current_number.clear();
                }
            }
        }

        sum
    }
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
    let matrix_opt: Option<EngineMatrix> = EngineMatrix::new(&file);
    let matrix: EngineMatrix = match matrix_opt {
        Some(x) => x,
        None => return Ok(()),
    };
    println!("Engine Sum: {}", matrix.engine_sum());

    Ok(())
}
