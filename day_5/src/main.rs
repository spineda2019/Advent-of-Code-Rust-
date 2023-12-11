use argparse::{ArgumentParser, Store};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: HashMap<usize, usize>,
    soil_to_fertilizer: HashMap<usize, usize>,
    fertilizer_to_water: HashMap<usize, usize>,
    water_to_light: HashMap<usize, usize>,
    light_to_temperature: HashMap<usize, usize>,
    temperature_to_humidity: HashMap<usize, usize>,
    humiduty_to_location: HashMap<usize, usize>,
}

impl Almanac {
    pub fn new(file: &File) -> Result<Almanac, std::io::Error> {
        #[derive(PartialEq)]
        enum MapType {
            SeedToSoil,
            SoilToFertilizer,
            FertilizerToWater,
            WaterToLight,
            LightToTemperature,
            TemperatureToHumidity,
            HumidutyToLocation,
            SeedGrab,
            Unspecified,
        }

        let mut seed_vector: Vec<usize> = Vec::new();
        let mut param_seed_to_soil: HashMap<usize, usize> = HashMap::new();
        let mut param_soil_to_fertilizer: HashMap<usize, usize> = HashMap::new();
        let mut param_fertilizer_to_water: HashMap<usize, usize> = HashMap::new();
        let mut param_water_to_light: HashMap<usize, usize> = HashMap::new();
        let mut param_light_to_temperature: HashMap<usize, usize> = HashMap::new();
        let mut param_temperature_to_humidity: HashMap<usize, usize> = HashMap::new();
        let mut param_humiduty_to_location: HashMap<usize, usize> = HashMap::new();

        let mut status: MapType = MapType::SeedGrab;

        let file_reader: BufReader<&File> = BufReader::new(file);
        for line in file_reader.lines() {
            let line = match line {
                Ok(x) => x,
                Err(y) => return Err(y),
            };

            if status == MapType::SeedGrab {
                let seeds: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
                for seed in seeds.iter().skip(1) {
                    seed_vector.push(seed.parse::<usize>().unwrap_or(0));
                }
                status = MapType::Unspecified;
                continue;
            }

            if line.chars().all(char::is_whitespace) {
                continue;
            } else if line.split_ascii_whitespace().collect::<Vec<&str>>()[1] == "map:" {
                match line.split_ascii_whitespace().collect::<Vec<&str>>()[0] {
                    "seed-to-soil" => status = MapType::SeedToSoil,
                    "soil-to-fertilizer" => status = MapType::SoilToFertilizer,
                    "fertilizer-to-water" => status = MapType::FertilizerToWater,
                    "water-to-light" => status = MapType::WaterToLight,
                    "light-to-temperature" => status = MapType::LightToTemperature,
                    "temperature-to-humidity" => status = MapType::TemperatureToHumidity,
                    "humidity-to-location" => status = MapType::HumidutyToLocation,
                    _ => continue,
                }
            } else {
                let data: Vec<&str> = line.split_ascii_whitespace().collect::<Vec<&str>>();
                let range: usize = data[2].parse::<usize>().unwrap_or(0);
                let source: usize = data[1].parse::<usize>().unwrap_or(0);
                let destination: usize = data[0].parse::<usize>().unwrap_or(0);

                for key in source..source + range {
                    match status {
                        MapType::SeedToSoil => {
                            let dest_value = destination + (key - source);
                            param_seed_to_soil.insert(key, dest_value);
                        }
                        MapType::SoilToFertilizer => {
                            let dest_value = destination + (key - source);
                            param_soil_to_fertilizer.insert(key, dest_value);
                        }
                        MapType::FertilizerToWater => {
                            let dest_value = destination + (key - source);
                            param_fertilizer_to_water.insert(key, dest_value);
                        }
                        MapType::WaterToLight => {
                            let dest_value = destination + (key - source);
                            param_water_to_light.insert(key, dest_value);
                        }
                        MapType::LightToTemperature => {
                            let dest_value = destination + (key - source);
                            param_light_to_temperature.insert(key, dest_value);
                        }
                        MapType::TemperatureToHumidity => {
                            let dest_value = destination + (key - source);
                            param_temperature_to_humidity.insert(key, dest_value);
                        }
                        MapType::HumidutyToLocation => {
                            let dest_value = destination + (key - source);
                            param_humiduty_to_location.insert(key, dest_value);
                        }
                        _ => todo!(),
                    }
                }
            }
        }

        Ok(Almanac {
            seeds: seed_vector,
            seed_to_soil: param_seed_to_soil,
            soil_to_fertilizer: param_soil_to_fertilizer,
            fertilizer_to_water: param_fertilizer_to_water,
            water_to_light: param_water_to_light,
            light_to_temperature: param_light_to_temperature,
            temperature_to_humidity: param_temperature_to_humidity,
            humiduty_to_location: param_humiduty_to_location,
        })
    }

    pub fn locations(&self) -> Vec<usize> {
        let mut locations: Vec<usize> = Vec::new();
        for seed in &self.seeds {
            let soil: &usize = match self.seed_to_soil.get(seed) {
                Some(x) => x,
                None => seed,
            };

            let fertilizer: &usize = match self.soil_to_fertilizer.get(soil) {
                Some(x) => x,
                None => soil,
            };

            let water: &usize = match self.fertilizer_to_water.get(fertilizer) {
                Some(x) => x,
                None => fertilizer,
            };

            let light: &usize = match self.water_to_light.get(water) {
                Some(x) => x,
                None => water,
            };

            let temperature: &usize = match self.light_to_temperature.get(light) {
                Some(x) => x,
                None => light,
            };

            let humidity: &usize = match self.temperature_to_humidity.get(temperature) {
                Some(x) => x,
                None => temperature,
            };

            let location: &usize = match self.humiduty_to_location.get(humidity) {
                Some(x) => x,
                None => humidity,
            };

            locations.push(*location);
        }
        locations
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
    let map: Almanac = Almanac::new(&file)?;

    println!("Constructed!");

    println!("Smallest Location: {:?}", map.locations().iter().min());

    Ok(())
}
