pub fn main() {
    todo!()
use std::collections::HashMap;
use std::fs;

pub fn part1(path_or_data: String) -> i32 {
    let data: String;
    if path_or_data.ends_with(".txt") {
        data = fs::read_to_string(path_or_data).expect("Error reading data file");
    } else {
        data = path_or_data;
    }

    let mut nums: Vec<i32> = vec![];

    for (_, content) in data.lines().enumerate() {
        // @@@ Personal challenge: no regex! @@@
        // We're looking for the first digit and the last digit of each line so 13abc46 would be 16 because 1 is the first digit and 6 the last.
        let mut first_num: Option<char> = None;
        let mut second_num: Option<char> = None;
        let filtered_content: Vec<char> = content.chars().filter(|c| c.is_ascii_digit()).collect();
        let total_length = filtered_content.len();

        // I really don't wanna use a nested for loop but this is my first time using Rust so idk if there is a better way.
        for (i, n) in filtered_content.into_iter().enumerate() {
            if total_length == 1 {
                nums.push(format!("{}{}", n, n).parse().expect("Failed to parse into number (i32)"));
                break;
            } else if first_num.is_none() {
                first_num = Some(n);
                // Check if this is the last element.
            } else if first_num.is_some() && second_num.is_none() && i == total_length - 1 {
                second_num = Some(n);
                nums.push(format!("{}{}", first_num.expect("Expected this to be defined"), n).parse().expect("Failed to parse into number (i32)"))
            }
        }
    }

    let final_num: i32 = nums.iter().sum();
    println!("The final sum is {}", final_num);
    return final_num;
}
}
