use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result: i32 = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {

        // pattern for digits
        let digit_pattern = &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let first_place = ip.find(digit_pattern);
                let last_place = ip.rfind(digit_pattern);

                let first_digit = ip.chars().nth(first_place.unwrap()).unwrap();
                let last_digit = ip.chars().nth(last_place.unwrap()).unwrap();

                let mut calibration_value_string = first_digit.to_string();
                calibration_value_string.push(last_digit);

                let calibration_value: i32 = calibration_value_string.parse().unwrap();

                result = result + calibration_value;
            }
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
