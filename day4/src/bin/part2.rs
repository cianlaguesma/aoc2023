use std::{collections::HashMap, fs, hash::Hash};
use itertools::Itertools;
fn main() {
    let contents = fs::read_to_string("input.txt");
    let mut hash_cards: HashMap<u32,u32> = HashMap::new();
    let mut total_arr:Vec<u32> = Vec::new();

    //First we should get how many paired numbers there are each card
    for line in contents.expect("Line not found").lines(){
        let mut split_game_num = line.split(":");
        let mut split_game_str = split_game_num.next().unwrap().split_whitespace();
        split_game_str.next();
        let game_num = split_game_str.next().unwrap().parse::<u32>().unwrap();
        let actual_sc_nums = (split_game_num.next().unwrap()).trim();
        let mut split_sc = actual_sc_nums.split("|");
        let winning_nums = (split_sc.next().unwrap()).trim();
        let my_nums = (split_sc.next().unwrap()).trim();
        let mut tot:u32 = 0;

        for w_nums in winning_nums.split_whitespace(){
            for num in my_nums.split_whitespace(){
                if w_nums == num {
                    tot += 1;
                }
            }
        }
        hash_cards.entry(game_num).or_insert(tot);
        total_arr.push(tot);
    }

    //After that, we put which cards are cloned. e.g. for 1, there are 4 cards, so the array for it should be [2,3,4,5]
    let cloned_hash_cards = hash_cards.clone();
    let mut prep_hash: HashMap<u32,Vec<u32>> = HashMap::new();
    for (key,value) in cloned_hash_cards.into_iter() {
        let mut iter:u32 = value;
        let hash_arr = prep_hash.entry(key).or_insert(vec![]);
        while iter > 0{
            hash_arr.push(iter+key);
            iter -=1;
        }
    }

    //After that, we now reverse it, start from the top and work your way down. Get the points from the top to bottom and it will just cascade.
    //So if 6 has 1 point, e.g. 5 has a clone 6, so 5 will have 2 points since it has a 5 card and a 6 card.
    let mut final_keys: Vec<&u32> = prep_hash.keys().collect();
    final_keys.sort();
    println!("{:?}",prep_hash);
    let mut hash_with_points: HashMap<u32, u32> = HashMap::new();
    for key in final_keys.iter().rev(){
        if let Some(value) = prep_hash.get(key) {
            println!("{:?}",value);
            //If card has no same number
            if value.len() == 0{
                hash_with_points.entry(**key).or_insert(1);
            }else{
                //If card has same numbers, resulting into cloned cards
                let point_getter = hash_with_points.clone();
                let points = hash_with_points.entry(**key).or_insert(1);
                for val in value{
                    *points += point_getter.get(val).unwrap();
                }
            }
        }
    }

    //Total all points
    let mut tot: u32 = 0;
    for (key,val) in hash_with_points {
        tot += val;
    }
    println!("{}",tot);
}

//do it in reverse
// 6 will have 1 point
// 5 will have 1 point
// 4 will have 2 points

// 1 = 2(7 pts),3(3),4(2),5 (1) = 14 pts
// 2 = 3(4),4(2) = 7 pts
// 3 = 4(2),5(1) = 4 pts
// 4 = 5 = 2 pts
// 5 = 1 pt
// 6 = 1pt

// 1 = 2(3(4(5),5),4(5)), 3(4(5),5), 4(5), 5
// 2 = 3(4(5),5), 4(5)
// 3 = 4(5), 5
// 4 = 5
// 5
// 6