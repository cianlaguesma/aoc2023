use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt");
    for line in contents.expect("Line not found").lines(){
    }
}
