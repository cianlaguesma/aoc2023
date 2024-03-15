use std::fs;
use std::collections::HashMap;
fn main() {
    let contents = fs::read_to_string("input.txt");
    let mut sum: u32 = 0;
    for line in contents.expect("Line not found").lines(){
        
        let mut color_hash : HashMap<&str,u32> = HashMap::new();
        let mut total : u32 = 0;
        //Current line
        let mut current = line.split(":");
        
        //Game ID
        let mut game_id_str: &str = current.next().unwrap();
        let mut game_id_u32: &u32 = &game_id_str.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        
        //Color Sentence after Game ID
        let sentence: String = current.next().unwrap().trim().to_string();
        //Split each batch delimited by ;
        let mut semi_colon_sentence = sentence.split(";");
        for sntn in semi_colon_sentence {
            //Each batch is sntn, ran out of words to describe
            let mut sntn_out = &sntn.trim(); 
            //Split each batch by , to get each number and color
            let mut color_number_split = sntn_out.split(",");
            for cn in color_number_split {
                //Counts each color and populates the hashmap
                let mut data = cn.trim().split(" ");
                let num = data.next().unwrap();
                let color = data.next().unwrap();

                //Retain max
                let mut out_hash = color_hash.entry(color).or_insert(0);
                if *out_hash < num.parse::<u32>().unwrap() {
                    *out_hash = num.parse::<u32>().unwrap();
                }
                }
            }
        let mut answer : u32 = 1;
        for (key,val) in color_hash{
            answer *= &val;
        }
        sum += answer;
    }
    println!("{}", sum);
}

