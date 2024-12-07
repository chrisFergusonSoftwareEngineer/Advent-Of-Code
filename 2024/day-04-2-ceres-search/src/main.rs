use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate regex;
use regex::Regex;

fn main() {
    let mut total = 0;

    let mut full_input = Vec::new();

    // probably needlessly complex unwrapping of line read-in
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line_string) = line {
                full_input.push(line_string);
            }
        }
    }

    for line_index in 1..full_input.len()-1 {
        let line = &full_input[line_index];
        println!("Line {} Length: {}: {}", line_index, line.len(), line);
        for char_index in 1..line.len()-1 {
            if line.chars().nth(char_index).unwrap() == 'A' {
                let first = full_input[line_index-1].chars().nth(char_index-1).unwrap();
                let second = full_input[line_index-1].chars().nth(char_index+1).unwrap();
                let third = full_input[line_index+1].chars().nth(char_index+1).unwrap();
                let fourth = full_input[line_index+1].chars().nth(char_index-1).unwrap();

                let mut orbit: String = String::new();
                orbit.push(first);
                orbit.push(second);
                orbit.push(third);
                orbit.push(fourth);

                if orbit == "MMSS" || orbit == "SMMS" || orbit == "SSMM" || orbit == "MSSM" {
                    total += 1;
                }
            }
        }
    }

    println!("{}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
