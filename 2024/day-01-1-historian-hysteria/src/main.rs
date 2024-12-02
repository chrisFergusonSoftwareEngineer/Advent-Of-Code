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

    list_a.sort();
    list_b.sort();

    let mut total_delta = 0;
    for index in 0..list_a.len() {
        let delta = list_a[index] - list_b[index];
        total_delta += delta.abs();
    }

    println!("{}", total_delta);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
