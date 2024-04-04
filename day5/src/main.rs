use std::{collections::HashMap, fs, thread::current};
fn main() {
    let contents = fs::read_to_string("input.txt");
    let lines = contents.expect("File not found");
    let mut lines_iter = lines.lines();
    let input = lines_iter.next();
    let mut input_spl = input.unwrap().split(":");
    input_spl.next();
    let seeds = input_spl.next().unwrap().trim();
    let mut seed_arr: Vec<i64> = Vec::new();

    // Add all seeds in an array
    for seed in seeds.split_whitespace(){
        seed_arr.push(seed.parse::<i64>().unwrap());
    }
    
    // Initialize min
    let mut min = seed_arr[0];

    // Iterate through each seed
    for seed in &seed_arr {
        let mut lines_iter = lines.lines();
        lines_iter.next();
        let mut num: i64 = *seed;
        let mut status: bool = false;

        // Iterate per line
        for line in lines_iter {
            let mut split_input = line.split_whitespace();
            if let Some(value) = split_input.next(){

                // If it is in a bridge-to-x string
                if value.parse::<f64>().is_err() {
                    status = true;
                } else if status {
                    // Get each range
                    let destination_range = value.parse::<i64>().unwrap();
                    let source_range = split_input.next().unwrap().parse::<i64>().unwrap();
                    let range_length = split_input.next().unwrap().parse::<i64>().unwrap();
                    
                    // Check if the current num is between the range
                    if num <= source_range + range_length && num >= source_range{

                        //If between the range, then that "BRIDGE" is done already, it shouldn't go to any other line
                        num = num + (destination_range - source_range);
                        status = false;

                    }

                }

            }
        }
        // Get minimum
        if min > num {
            min = num;
        }
    }
    println!("Min: {}",min);
    
}
