use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(XMAS|SMAX)").unwrap();
    
    let mut total = 0;

    let mut full_input = Vec::new();

    // probably needlessly complex unwrapping of line read-in
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line_string) = line {
    
                for (_, [_]) in re.captures_iter(&line_string).map(|c| c.extract()) {
                    total += 1;
                }

                full_input.push(line_string);
            }
        }
    }

    println!("Initial Total: {}", total);
    println!("Full Input Length: {}", full_input.len());

    for line_index in 0..full_input.len() {
        let line = &full_input[line_index];
        println!("Line {} Length: {}", line_index, line.len());
        for char_index in 0..line.len() {
            // println!("Char {} is {} of line {}", char_index, line.chars().nth(char_index).unwrap(), line);
            if line.chars().nth(char_index).unwrap() == 'X' {
                if line_index >= 3 { //search UP
                    if (full_input[line_index-1].chars().nth(char_index).unwrap() == 'M') &&
                       (full_input[line_index-2].chars().nth(char_index).unwrap() == 'A') &&
                       (full_input[line_index-3].chars().nth(char_index).unwrap() == 'S') {
                        total += 1; //vertical UP
                       }
                    
                    if char_index >= 3 { //diagonal up left
                        if (full_input[line_index-1].chars().nth(char_index-1).unwrap() == 'M') &&
                           (full_input[line_index-2].chars().nth(char_index-2).unwrap() == 'A') &&
                           (full_input[line_index-3].chars().nth(char_index-3).unwrap() == 'S') {
                            total += 1; // UP - LEFT
                        }
                    }

                    if char_index <= line.len()-4 { //diagonal up right
                        if (full_input[line_index-1].chars().nth(char_index+1).unwrap() == 'M') &&
                           (full_input[line_index-2].chars().nth(char_index+2).unwrap() == 'A') &&
                           (full_input[line_index-3].chars().nth(char_index+3).unwrap() == 'S') {
                            total += 1; // UP - RIGHT
                        }
                    }
                }

                if line_index <= full_input.len()-4 { //search DOWN
                    if (full_input[line_index+1].chars().nth(char_index).unwrap() == 'M') &&
                       (full_input[line_index+2].chars().nth(char_index).unwrap() == 'A') &&
                       (full_input[line_index+3].chars().nth(char_index).unwrap() == 'S') {
                        total += 1; //vertical DOWN
                       }
                    
                    if char_index >= 3 { //diagonal down left
                        if (full_input[line_index+1].chars().nth(char_index-1).unwrap() == 'M') &&
                           (full_input[line_index+2].chars().nth(char_index-2).unwrap() == 'A') &&
                           (full_input[line_index+3].chars().nth(char_index-3).unwrap() == 'S') {
                            total += 1; // DOWN - LEFT
                        }
                    }

                    if char_index <= line.len()-4 { //diagonal down right
                        if (full_input[line_index+1].chars().nth(char_index+1).unwrap() == 'M') &&
                           (full_input[line_index+2].chars().nth(char_index+2).unwrap() == 'A') &&
                           (full_input[line_index+3].chars().nth(char_index+3).unwrap() == 'S') {
                            total += 1; // DOWN - RIGHT
                        }
                    }
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
