use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(XMAS)").unwrap();
    // This annoyed me for far too long.  But REGEX cannot do overlapping matches: "XMASAMX"  So I need to search twice, separately.
    let backward = Regex::new(r"(SAMX)").unwrap();
    
    let mut total = 0;

    let mut full_input = Vec::new();

    // probably needlessly complex unwrapping of line read-in
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line_string) = line {
    
                total += re.find_iter(&line_string).count();
                total += backward.find_iter(&line_string).count();

                full_input.push(line_string);
            }
        }
    }

    println!("Initial Total: {}", total);
    println!("Full Input Length: {}", full_input.len());

    let mut up_finds = 0;
    let mut up_left_finds = 0;
    let mut up_right_finds = 0;
    let mut down_finds = 0;
    let mut down_left_finds = 0;
    let mut down_right_finds = 0;

    for line_index in 0..full_input.len() {
        let line = &full_input[line_index];
        println!("Line {} Length: {}: {}", line_index, line.len(), line);
        for char_index in 0..line.len() {
            if line.chars().nth(char_index).unwrap() == 'X' {
                if line_index >= 3 { //search UP
                    if (full_input[line_index-1].chars().nth(char_index).unwrap() == 'M') &&
                       (full_input[line_index-2].chars().nth(char_index).unwrap() == 'A') &&
                       (full_input[line_index-3].chars().nth(char_index).unwrap() == 'S') {
                        up_finds += 1; //vertical UP
                       }
                    
                    if char_index >= 3 { //diagonal up left
                        if (full_input[line_index-1].chars().nth(char_index-1).unwrap() == 'M') &&
                           (full_input[line_index-2].chars().nth(char_index-2).unwrap() == 'A') &&
                           (full_input[line_index-3].chars().nth(char_index-3).unwrap() == 'S') {
                            up_left_finds += 1; // UP - LEFT
                        }
                    }

                    if char_index <= line.len()-4 { //diagonal up right
                        if (full_input[line_index-1].chars().nth(char_index+1).unwrap() == 'M') &&
                           (full_input[line_index-2].chars().nth(char_index+2).unwrap() == 'A') &&
                           (full_input[line_index-3].chars().nth(char_index+3).unwrap() == 'S') {
                            up_right_finds += 1; // UP - RIGHT
                        }
                    }
                }

                if line_index <= full_input.len()-4 { //search DOWN
                    if (full_input[line_index+1].chars().nth(char_index).unwrap() == 'M') &&
                       (full_input[line_index+2].chars().nth(char_index).unwrap() == 'A') &&
                       (full_input[line_index+3].chars().nth(char_index).unwrap() == 'S') {
                        down_finds += 1; //vertical DOWN
                       }
                    
                    if char_index >= 3 { //diagonal down left
                        if (full_input[line_index+1].chars().nth(char_index-1).unwrap() == 'M') &&
                           (full_input[line_index+2].chars().nth(char_index-2).unwrap() == 'A') &&
                           (full_input[line_index+3].chars().nth(char_index-3).unwrap() == 'S') {
                            down_left_finds += 1; // DOWN - LEFT
                        }
                    }

                    if char_index <= line.len()-4 { //diagonal down right
                        if (full_input[line_index+1].chars().nth(char_index+1).unwrap() == 'M') &&
                           (full_input[line_index+2].chars().nth(char_index+2).unwrap() == 'A') &&
                           (full_input[line_index+3].chars().nth(char_index+3).unwrap() == 'S') {
                            down_right_finds += 1; // DOWN - RIGHT
                        }
                    }
                }
            }
        }
    }

    println!("Search Finds: UP: {}", up_finds);
    println!("Search Finds: UP-LEFT: {}", up_left_finds);
    println!("Search Finds: UP-RIGHT: {}", up_right_finds);
    println!("Search Finds: DOWN: {}", down_finds);
    println!("Search Finds: DOWN-LEFT: {}", down_left_finds);
    println!("Search Finds: DOWN-RIGHT: {}", down_right_finds);
    total += (up_finds + up_left_finds + up_right_finds + down_finds + down_left_finds + down_right_finds);
    
    println!("{}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
