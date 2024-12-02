use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result: i64 = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {

        // pattern for digits
        let digit_pattern = &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_string) = line {
                let ip = convert_words_to_digits(line_string);
                println!("{}", ip);

                let first_place = ip.find(digit_pattern).unwrap();
                let last_place = ip.rfind(digit_pattern).unwrap();

                let first_digit = ip.chars().nth(first_place).unwrap();
                let last_digit = ip.chars().nth(last_place).unwrap();

                let mut calibration_value_string = first_digit.to_string();
                calibration_value_string.push(last_digit);

                let calibration_value: i64 = calibration_value_string.parse().unwrap();

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

fn convert_words_to_digits(input_string: String) -> String {
    let ones_result = input_string.replace("one", "o1ne");

    let twos_result = ones_result.replace("two", "t2wo");

    let threes_result = twos_result.replace("three", "th3ree");

    let fours_result = threes_result.replace("four", "fo4ur");

    let fives_result = fours_result.replace("five", "fi5ve");

    let sixs_result = fives_result.replace("six", "s6ix");

    let sevens_result = sixs_result.replace("seven", "se7ven");

    let eights_result = sevens_result.replace("eight", "ei8ght");

    let result = eights_result.replace("nine", "ni9ne");

    return result;
}
