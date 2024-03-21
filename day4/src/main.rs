use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt");
    let mut tot: u32 = 0;
    for line in contents.expect("Line not found").lines(){
        let mut split_game_num = line.split(":");
        split_game_num.next();
        let actual_sc_nums = (split_game_num.next().unwrap()).trim();
        let mut split_sc = actual_sc_nums.split("|");
        let winning_nums = (split_sc.next().unwrap()).trim();
        let my_nums = (split_sc.next().unwrap()).trim();
        println!("{} {}",winning_nums,my_nums);
        let mut base: u32 = 2;
        let mut exp: i32 = -1;
        for w_nums in winning_nums.split_whitespace(){
            for num in my_nums.split_whitespace(){
                if w_nums == num {
                    // println!("{}",num);
                    exp += 1;
                }
            }
        }
        if exp > -1 {
            println!("{}",exp);
            tot += base.pow(exp.try_into().unwrap());
        }
    }
    println!("{}",tot);

}
