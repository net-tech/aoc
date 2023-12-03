use crate::common::get_data_from_file_or_str;
use std::collections::HashMap;

pub fn part1(path_or_data: String) -> i32 {
    let data = get_data_from_file_or_str(path_or_data);

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
                nums.push(
                    format!("{}{}", n, n)
                        .parse()
                        .expect("Failed to parse into number (i32)"),
                );
                break;
            } else if first_num.is_none() {
                first_num = Some(n);
                // Check if this is the last element.
            } else if first_num.is_some() && second_num.is_none() && i == total_length - 1 {
                second_num = Some(n);
                nums.push(
                    format!("{}{}", first_num.expect("Expected this to be defined"), n)
                        .parse()
                        .expect("Failed to parse into number (i32)"),
                )
            }
        }
    }

    let final_num: i32 = nums.iter().sum();
    println!("The final sum is {}", final_num);
    return final_num;
}

pub fn part2(path_or_data: String) -> i32 {
    let data = get_data_from_file_or_str(path_or_data);

    let mut nums: Vec<i32> = vec![];
    let number_word_equivalents: HashMap<&str, &str> = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    for (_, content) in data.lines().enumerate() {
        // @@@ Personal challenge: no regex! @@@
        let mut first_num: Option<char> = None;
        let mut second_num: Option<char> = None;

        let mut result = String::new();
        let mut stored_char_info: Option<(char, usize)> = None;
        let chars = content.chars().collect::<Vec<char>>();

        for (i, &char) in chars.iter().enumerate() {
            // Insert the character which was last removed
            if let Some((stored_char, _)) = stored_char_info {
                result.push(stored_char);
                stored_char_info = None;
            }

            result.push(char);
            for (key, value) in number_word_equivalents.iter() {
                if result.ends_with(key) {
                    result = result.replace(key, value);
                    // Now lets update the stored_char_info; check is to prevent subtraction underflow
                    if i >= key.len() {
                        let new_pos = i - key.len() + value.len();
                        stored_char_info = Some((char, new_pos));
                    }
                }
            }
        }

        let filtered_content: Vec<char> = result.chars().filter(|c| c.is_ascii_digit()).collect();
        let total_length = filtered_content.len();

        // I really don't wanna use a nested for loop but this is my first time using Rust so idk if there is a better way.
        for (i, n) in filtered_content.into_iter().enumerate() {
            if total_length == 1 {
                nums.push(
                    format!("{}{}", n, n)
                        .parse()
                        .expect("Failed to parse into number (i32)"),
                );
                break;
            } else if first_num.is_none() {
                first_num = Some(n);
                // Check if this is the last element.
            } else if first_num.is_some() && second_num.is_none() && i == total_length - 1 {
                second_num = Some(n);
                nums.push(
                    format!("{}{}", first_num.expect("Expected this to be defined"), n)
                        .parse()
                        .expect("Failed to parse into number (i32)"),
                )
            }
        }
    }

    let final_num: i32 = nums.iter().sum();
    println!("The final sum is {}", final_num);
    final_num
}

// Just testing part two
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_at_the_start_and_end() {
        let data = String::from("1abc2");
        let result = part2(data);
        assert_eq!(result, 12);
    }

    #[test]
    fn digits_in_the_middle() {
        let data = String::from("pqr3stu8vwx");
        let result = part2(data);
        assert_eq!(result, 38);
    }

    #[test]
    fn multiple_digits_in_the_middle() {
        let data = String::from("a1b2c3d4e5f");
        let result = part2(data);
        assert_eq!(result, 15);
    }

    #[test]
    fn one_digit_in_the_middle() {
        let data = String::from("treb7uchet");
        let result = part2(data);
        assert_eq!(result, 77);
    }

    #[test]
    fn word_number_at_the_start_and_end() {
        let data = String::from("two1nine");
        let result = part2(data);
        assert_eq!(result, 29);
    }

    #[test]
    fn word_number_overlapping() {
        let data = String::from("eightwothree");
        let result = part2(data);
        assert_eq!(result, 83);
    }

    #[test]
    fn word_number_and_digit_() {
        let data = String::from("abcone2threexyz");
        let result = part2(data);
        assert_eq!(result, 13);
    }

    #[test]
    fn word_numbers_and_digit_overlapping() {
        let data = String::from("xtwone3four");
        let result = part2(data);
        assert_eq!(result, 24);
    }

    #[test]
    fn digits_at_the_start_and_end_with_word_numbers() {
        let data = String::from("4nineeightseven2");
        let result = part2(data);
        assert_eq!(result, 42);
    }

    #[test]
    fn word_number_at_the_start_and_end_with_digits() {
        let data = String::from("zoneight234");
        let result = part2(data);
        assert_eq!(result, 14);
    }

    #[test]
    fn digit_at_the_start_and_end_with_word_numbers() {
        let data = String::from("7pqrstsixteen");
        let result = part2(data);
        assert_eq!(result, 76);
    }

    #[test]
    fn digit_at_the_start_with_word_numbers() {
        let data = String::from("1eightwo");
        let result = part2(data);
        assert_eq!(result, 12);
    }
}
