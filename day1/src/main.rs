use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt");
    let mut sum: u32 = 0;
    for line in contents.expect("Line not found").lines(){
        let mut numArr: Vec<char> = Vec::new();
        println!("{}",line);
        for char in line.chars(){
            if char.is_numeric(){
                numArr.push(char);
            }
        }
        sum += (numArr[0].to_string() + &numArr[numArr.len()-1].to_string()).parse::<u32>().unwrap();
        println!("{}",sum);
    }
}
