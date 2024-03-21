use std::{collections::HashMap, fs, hash::Hash};
use itertools::Itertools;
fn main() {
    let contents = fs::read_to_string("input.txt");
    let mut hash_cards: HashMap<u32,u32> = HashMap::new();

    // Get the number of duplicated cards in each card block
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
    }
    // Init your vector with the length of how many card blocks
    let mut init_point_arr:  Vec<u32> = vec![0;hash_cards.len()];
    // Sort the cards first
    let mut sorted_hash_cards: Vec<&u32> = hash_cards.keys().collect();
    sorted_hash_cards.sort();

    // Now, you just iterate through each card block
    // e.g. if 1 has 2 3 4 5, you add how many cards of 1 you have to 2, 3, 4, 5. So you add 1 to the array indices of 2,3,4,5
    // Now, you are in index 2, you have 1 duplicated 2 card PLUS 1 original 2 card. Now, you add 2 to indices 3,4. so 3,4 is now [3,3] respectively.
    // Continue then you will get the desired values
    for key in sorted_hash_cards.iter(){
        if let Some(value) = hash_cards.get(key){
            // Add the original card to the current index, because you always have 1 original card
            init_point_arr[**key as usize - 1] += 1;

            // Now iterate through the duplicated cards and add your value to it
            for x in **key+1..value+**key+1{
                init_point_arr[x as usize -1] += init_point_arr[**key as usize - 1];
            }
        }
    }

    // Sum, ez
    let sum = init_point_arr.into_iter().reduce(|a,b| a+b);
    println!("{:?}",sum.unwrap());
    // 1 = 2,3,4,5
    // 2 = 3,4
    // 3 = 4,5
    // 4 = 5
    // 5
    // 6
    //[1,0,0,0,0,0]
    // [1,2,4,8,14,1]
}