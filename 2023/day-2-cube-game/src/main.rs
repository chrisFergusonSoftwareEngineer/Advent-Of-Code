use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result: i32 = 0;

    let red_load = 12;
    let green_load = 13;
    let blue_load = 14

    if let Ok(lines) = read_lines("./src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
             
                let mut red_shown = 0;
                let mut green_shown = 0;
                let mut blue_shown = 0;

                

                if (red_shown <= red_load) && (green_shown <= green_load) && (blue_shown <= blue_load) {
                    result += game_number;
                }
            
            } // end line reading inner loop
        }
    }

    println!("result: {}", result);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
