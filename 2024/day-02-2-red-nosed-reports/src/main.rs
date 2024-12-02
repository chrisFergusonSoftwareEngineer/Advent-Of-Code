use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate regex;
use regex::Regex;

fn main() {
    let mut safe_count = 0;
    
    let re = Regex::new(r"(\d+)").unwrap();
    
    // probably needlessly complex unwrapping of line read-in
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line_string) = line {
    
                let mut previous_reading = -1;
                let mut trend = 0;
                let mut reading_count = 0;

                let mut safe = true;

                for (_, [reading]) in re.captures_iter(&line_string).map(|c| c.extract()) {
                    reading_count += 1;
                    if previous_reading == -1 {
                        previous_reading = reading.parse().unwrap();
                    } else {
                        let delta: i32 = previous_reading - (reading.parse::<i32>().unwrap());
                        if delta.abs() > 3 || delta.abs() < 1 {
                            safe = false;
                            println!("DELTA: {}", line_string);
                        }
                        if trend == 0 {
                            trend = delta;
                        } else {
                            if (trend > 0 && delta < 0) || (trend < 0 && delta > 0) {
                                safe = false;
                                println!("TREND: {}", line_string);
                            }
                        }
                        previous_reading = reading.parse::<i32>().unwrap();
                    }
                }

                if !safe {
                    let mut skip_index = 0;
                    while !safe && (skip_index < reading_count) {
                        let mut recheck_safe = true;
                        let mut previous_reading = -1;
                        let mut trend = 0;

                        let mut reading_index = 0;
                        for (_, [reading]) in re.captures_iter(&line_string).map(|c| c.extract()) {
                            if reading_index != skip_index {
                                if previous_reading == -1 {
                                    previous_reading = reading.parse().unwrap();
                                } else {
                                    let delta: i32 = previous_reading - (reading.parse::<i32>().unwrap());
                                    if delta.abs() > 3 || delta.abs() < 1 {
                                        recheck_safe = false;
                                        println!("DELTA: {}", line_string);
                                    }
                                    if trend == 0 {
                                        trend = delta;
                                    } else {
                                        if (trend > 0 && delta < 0) || (trend < 0 && delta > 0) {
                                            recheck_safe = false;
                                            println!("TREND: {}", line_string);
                                        }
                                    }
                                    previous_reading = reading.parse::<i32>().unwrap();
                                }
                            }
                            reading_index += 1;
                        }

                        safe = recheck_safe;
                        skip_index += 1;
                    }
                }

                if safe {
                    safe_count += 1;
                }

            }
        }
    }

    println!("{}", safe_count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
