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
    // MAYBE ADD ALL NUMBERS BELOW AND GET ALL NUMBERS IN BETWEEN, IDK
    let mut previous_num:i64 = 0;
    for (mut idx, seed) in seeds.split_whitespace().enumerate(){
        let curr_num = seed.parse::<i64>().unwrap();
        idx += 1;
        if idx % 2 == 0{
            for x in previous_num..previous_num+curr_num{
                seed_arr.push(x);
            }
        } else {
            previous_num = curr_num;
        }
        
    }
    println!("{:?}",seed_arr);
    
    // Initialize min
    let mut min = seed_arr[0];

    // Iterate through each seed
    for seed in &seed_arr {
        let mut map: HashMap<&str,i64> = HashMap::new();
        let mut lines_iter = lines.lines();
        lines_iter.next();
        let mut curr_map: &str = "";
        let mut previous_map: &str = "";
        let mut current_num = *seed;

        // Iterate per line
        for line in lines_iter {
            let mut split_input = line.split_whitespace();
            if let Some(value) = split_input.next(){

                // If it is in a bridge-to-x string
                if value.parse::<f64>().is_err() {
                    // Then just add its value to the previous map
                    map.entry(previous_map).or_insert(current_num);
                    
                    // Now the current map is the current string
                    curr_map = value;
                } else {
                    // Get each range
                    let destination_range = value.parse::<i64>().unwrap();
                    let source_range = split_input.next().unwrap().parse::<i64>().unwrap();
                    let range_length = split_input.next().unwrap().parse::<i64>().unwrap();
                    previous_map = curr_map;
                    
                    // Check if the current num is between the range
                    if current_num <= source_range + range_length && current_num >= source_range{

                        //If it is, then it should be the value inside the hashmap, no other value will be taken even if it goes inside here again
                        map.entry(curr_map).or_insert(current_num + (destination_range - source_range));
                        //Update current num
                        current_num = *map.get(curr_map).unwrap();
                    }

                }

            }
        }

        map.entry(curr_map).or_insert(current_num);

        // Get minimum
        if min > *map.get("humidity-to-location").unwrap() {
            min = *map.get("humidity-to-location").unwrap();
        }
    }
    println!("Min: {}",min);
    
}
