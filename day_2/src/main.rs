use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Invalid file path!");

    let contents = contents.lines();
    let mut return_number = 0;

    // a given game
    for content in contents {
        let mut red_max = 0;
        let mut blue_max = 0;
        let mut green_max = 0;
        let parts: Vec<&str> = content.split(':').collect();
        let pulls: Vec<&str> = parts[1].split(';').collect();

        // a given pull
        for pull in pulls {
            let pulled_per_color = pull.trim().split(',');
            // given color for a pull
            for color in pulled_per_color {
                let color = color.trim();
                let color_parts: Vec<&str> = color.split_whitespace().collect();
                let color_amount: i32 = color_parts[0].trim().parse().expect("Should be an integer");
                match color_parts[1].trim() {
                    "red" => if color_amount > red_max { red_max = color_amount },
                    "blue" => if color_amount > blue_max { blue_max = color_amount },
                    "green" => if color_amount > green_max { green_max = color_amount },
                    _ => { panic!("Something went wrong, non-valid color found!"); }
                }
            }
        }
        let power = red_max * blue_max * green_max;
        return_number += power;
        
    }
    println!("{}", return_number);
    
}
