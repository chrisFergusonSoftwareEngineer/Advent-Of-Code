use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::Path;

extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+)(?:\|)(\d+)").unwrap();
    let pages_re = Regex::new(r"(\d+)(?:,|\n)").unwrap();
    
    let mut total = 0;

    let mut reading_rules = true;

    let mut firsts: Vec<i32> = Vec::new();
    let mut seconds: Vec<i32> = Vec::new();

    // probably needlessly complex unwrapping of line read-in
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line_string) = line {
                if reading_rules {
                    if line_string == "" {
                        println!("END OF RULES");
                        reading_rules = false;
                    } else {
                        for (_, [first, second]) in re.captures_iter(&(line_string)).map(|c| c.extract()) {
                            firsts.push(first.parse().unwrap());
                            seconds.push(second.parse().unwrap());
                            println!("LINE: {} ; RULE{}: {} | {}", line_string, firsts.len(), first, second); 
                        }
                    }
                } else {
                    let mut pages: Vec<i32> = Vec::new();
                    for (_, [page_str]) in pages_re.captures_iter(&(line_string)).map(|c| c.extract()) {
                        pages.push(page_str.parse().unwrap());
                    }

                    let mut valid = true;

                    for index in 0..seconds.len() {
                        if valid {
                            let second_place = pages.iter().position(|&r| r == seconds[index]);
                            let first_place = pages.iter().position(|&r| r == firsts[index]);

                            if second_place != None && first_place != None {
                                if second_place.unwrap() < first_place.unwrap() {
                                    println!("failed on rule {}: {} | {}", index, firsts[index], seconds[index]);
                                    valid = false;
                                }
                            }
                        }
                    }

                    if valid {
                        let middle = pages.len() / 2;
                        total += pages[middle];
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
