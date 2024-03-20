use std::fs;

#[derive(Debug)]
struct Number{
    start: u32,
    end: u32,
    row: u32,
    value: i32,
}
fn main() {
    let contents = fs::read_to_string("input.txt");
    let mut row_arr: Vec<Vec<String>> = Vec::new();
    // Transfer numbers to array for easier manipulation
    for line in contents.expect("Line not found").lines(){
        let mut letter_arr: Vec<String> = Vec::new();
        for char in line.chars(){
            let letter_arr_size = letter_arr.len();
            if char.is_numeric() && letter_arr_size != 0{
                println!("{}",char);
                if &letter_arr[letter_arr_size-1] != "." && letter_arr[letter_arr_size-1].parse::<f64>().is_ok(){
                    letter_arr.push(format!("{}{}",&letter_arr[letter_arr_size - 1], char));
                    letter_arr[letter_arr_size-1] = ".".to_string();
                } else {
                    letter_arr.push(char.to_string());
                }
            } else {
                letter_arr.push(char.to_string());
            }
        }
        row_arr.push(letter_arr);
    }

    let mut num_arr: Vec<Number> = Vec::new();
    // Initialize number struct
    for (idx_row,row) in row_arr.iter().enumerate() {
        for (idx_col, col) in row.iter().enumerate(){
            if col.parse::<f64>().is_ok(){
                let start: u32 = (idx_col as u32) - ((col.len() as u32) - 1);
                let end: u32 = idx_col as u32;
                let row: u32 = idx_row as u32;
                let number_ins = Number { start, end, row, value: col.parse::<i32>().unwrap()};
                num_arr.push(number_ins);
                // println!("{} {} {}",col, start, end);
            }
        }
    }

    let mut total: i32 =0;
    for (idx_row,row) in row_arr.iter().enumerate() {
        for (idx_col, col) in row.iter().enumerate(){
            if col.parse::<f64>().is_err() && col != "." {
                println!("Row: {} Col: {}", idx_row, idx_col);
                for row in idx_row-1..idx_row+2{
                    for col in idx_col-1..idx_col+2{
                        let mut indices_to_remove: Vec<usize> = Vec::new();
                        for (idx_num, num) in num_arr.iter().enumerate() {
                            if num.end >= col as u32 && num.start <= col as u32 && num.row == row as u32{
                                println!("{}",num.value);
                                indices_to_remove.push(idx_num);
                            }
                        }
                        for idx in indices_to_remove.into_iter().rev(){
                            total += num_arr[idx].value;
                            num_arr.remove(idx);
                        }
                    }
                }
            }
        }
    }
    println!("{}",total);
    // println!("{:?}",num_arr);
}

