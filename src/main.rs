mod day1;

fn main() {
    let day = std::env::args().nth(1).expect("Please specify the day to run");

    match day.as_str() {
        "1" => day1::main(),
        _ => panic!("Unknown day: {}", day),
    }
}
