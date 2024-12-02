use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+)(?: +)(\d+)").unwrap();
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();

    // probably needlessly complex unwrapping of line read-in
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line_string) = line {
    
                for (_, [list_item_a, list_item_b]) in re.captures_iter(&line_string).map(|c| c.extract()) {
                    list_a.push(list_item_a.parse().unwrap());
                    list_b.push(list_item_b.parse().unwrap());
                }

            }
        }
    }

    let mut similarity_score = 0;

    for left_number in list_a {
        let multiplier = list_b.iter().filter(|&n| *n == left_number).count() as i32;
        similarity_score += (left_number * multiplier);
    }

    println!("{}", similarity_score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
