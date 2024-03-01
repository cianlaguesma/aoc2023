use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum Number{
    IntegerText(char),
    StringText(String)
}
fn main() {
    let contents = fs::read_to_string("input2.txt");
    let mut sum: u32 = 0;
    let num_strings: Vec<&str> = ["zero","one","two","three","four","five","six","seven","eight","nine"].to_vec();
    let num_ints: Vec<u32> = [0,1,2,3,4,5,6,7,8,9].to_vec();
    let mut string_conv_num: HashMap<&str, u32> = HashMap::new();

    //Create Hashmap
    for (idx,num) in num_ints.iter().enumerate(){
        string_conv_num.insert(num_strings[idx],*num);
    }
    println!("{:?}",string_conv_num);

    for line in contents.expect("Line not found").lines(){
        let mut numArr: Vec<Number> = Vec::new();
        let mut builderArr: Vec<String> = Vec::new();

        for char in line.chars(){
            let character = char.to_string();
            builderArr.push(character);

            if char.is_digit(10){
                numArr.push(Number::IntegerText(char));
                builderArr = Vec::new();
            }

            for num in &num_strings{
                let check = builderArr.join("");
                if check.contains(num){
                    numArr.push(Number::StringText(num.to_string()));
                    builderArr = vec![(&num[(num.len()-1)..]).to_string()];
                    println!("{:?}",builderArr);
                }
            }
        }
        println!("{:?}, {:?}",&numArr[0], &numArr[numArr.len()-1]);
        let firstNum = match &numArr[0] {
            Number::StringText(x) => string_conv_num.get(x.as_str()).unwrap().to_string(),
            Number::IntegerText(x) => x.to_string()
        };
        let secondNum = match &numArr[numArr.len()-1]{
            Number::StringText(x) => string_conv_num.get(x.as_str()).unwrap().to_string(),
            Number::IntegerText(x) => x.to_string()
        };
        let answer = (firstNum + &secondNum).parse::<u32>().unwrap();
        sum += &answer;
        println!("{:?}",&answer);
        // sum += ( (string_conv_num.get(&numArr[0]).unwrap()).to_string() + &(string_conv_num.get(&numArr[numArr.len()-1]).unwrap()).to_string() ).parse::<u32>().unwrap();
        // println!("{}",sum);
    }
    println!("{}",&sum);

}
