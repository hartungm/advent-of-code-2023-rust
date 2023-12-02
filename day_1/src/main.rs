use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Invalid file path!");

    let contents = contents.split("\n");

    let mut calibration_values: Vec<i32> = Vec::new();

    for content in contents {
        let calibration_value = get_calibration_value_from_string(content);
        println!("{calibration_value}");
        calibration_values.push(calibration_value);
    }

    let mut calibration_total = 0;

    for value in calibration_values {
        calibration_total = calibration_total + value;
    }

    println!("{calibration_total}");
}

fn get_calibration_value_from_string(string: &str) -> i32 {
    let mut calibration_digits: Vec<char> = Vec::new();

    let string = string.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    
    for character in string.chars() {
        if character.is_numeric() {
            calibration_digits.push(character);
        }
    }

    let calibration_value = {
        format!("{}{}", calibration_digits[0], calibration_digits[calibration_digits.len() - 1])
    };

    return calibration_value.trim().parse().expect("Parsed value must be a number!");
}
