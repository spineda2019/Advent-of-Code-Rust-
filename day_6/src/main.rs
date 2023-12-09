use argparse::{ArgumentParser, Store};
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    pub fn new(time_param: usize, distance_param: usize) -> Race {
        Race {
            time: time_param,
            distance: distance_param,
        }
    }

    pub fn possible_wins(&self) -> usize {
        let mut wins: usize = 0;
        for seconds_held in 1..self.time {
            if seconds_held * (self.time - seconds_held) > self.distance {
                wins += 1;
            }
        }
        wins
    }
}

fn add_race(race_array: &mut Vec<Race>, time: &str, distance: &str) -> Result<(), std::io::Error> {
    let time_param: Result<usize, std::num::ParseIntError> = time.parse();
    let time_param: usize = match time_param {
        Ok(x) => x,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "Number not parsed correctly",
            ))
        }
    };

    let distance_param: Result<usize, std::num::ParseIntError> = distance.parse();
    let distance_param: usize = match distance_param {
        Ok(x) => x,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "Number not parsed correctly",
            ))
        }
    };
    race_array.push(Race::new(time_param, distance_param));
    Ok(())
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

    let file: std::fs::File = std::fs::File::open(file_name)?;
    let file_reader: BufReader<&std::fs::File> = BufReader::new(&file);
    let mut races: Vec<Race> = Vec::new();
    let mut times: Vec<String> = Vec::new();
    let mut distances: Vec<String> = Vec::new();

    for line in file_reader.lines() {
        match line {
            Ok(x) => {
                let tokens = x.split_whitespace().collect::<Vec<&str>>();
                match tokens[0] {
                    "Time:" => {
                        for token in tokens.iter().skip(1) {
                            times.push(token.to_string());
                        }
                    }
                    "Distance:" => {
                        for token in tokens.iter().skip(1) {
                            distances.push(token.to_string());
                        }
                    }
                    &_ => continue,
                }
            }
            Err(_) => continue,
        }
    }

    println!("Times: {:?}\nDistances: {:?}", times, distances);

    for index in 0..times.len() {
        add_race(&mut races, &times[index], &distances[index])?;
    }

    println!("Races: {:?}", races);

    let mut wins: Vec<usize> = Vec::new();

    for race in races {
        wins.push(race.possible_wins());
    }

    println!("Wins: {}", wins.iter().product::<usize>());

    let big_time: String = times.concat();
    let big_distance: String = distances.concat();

    let big_time = match big_time.parse::<usize>() {
        Ok(x) => x,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "Number not parsed correctly",
            ))
        }
    };

    let big_distance = match big_distance.parse::<usize>() {
        Ok(x) => x,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "Number not parsed correctly",
            ))
        }
    };

    let big_race = Race::new(big_time, big_distance);
    println!("One Win: {}", big_race.possible_wins());

    Ok(())
}
