use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(?:mul\()(\d+)(?:,)(\d+)(?:\))").unwrap();
    
    let mut total = 0;

    // probably needlessly complex unwrapping of line read-in
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line_string) = line {
    
                for (_, [first, second]) in re.captures_iter(&line_string).map(|c| c.extract()) {
                    total += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
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
