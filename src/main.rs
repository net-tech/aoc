use std::time::Instant;

mod day1;
mod day2;
mod common;

fn main() {
    let day = std::env::args().nth(1).expect("Please specify the day to run");
    let part = std::env::args().nth(2).expect("Please specify the part to run");

    println!("---------------{}.{}---------------", day, part);

    match (day.as_str(), part.as_str()) {
        ("1", "1") => {
            let start = Instant::now();
            day1::part1("./data/day1/data.txt".to_string());
            println!("Time elapsed: {:?}", start.elapsed());
        }
        ("1", "2") => {
            let start = Instant::now();
            day1::part2("./data/day1/data.txt".to_string());
            println!("Time elapsed: {:?}", start.elapsed());
        }
        ("2", "1") => {
            let start = Instant::now();
            day2::part1("./data/day2/data.txt".to_string());
            println!("Time elapsed: {:?}", start.elapsed());
        }
        ("2", "2") => {
            let start = Instant::now();
            day2::part2("./data/day2/data.txt".to_string());
            println!("Time elapsed: {:?}", start.elapsed());
        }
        _ => println!("Challenge for day {} part {} not found", day, part),
    }
}
