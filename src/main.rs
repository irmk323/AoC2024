mod solutions {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
}
mod utils; 

fn main() {
    let day = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);  // default to day 1

    match day {
        1 => solutions::day1::solve(day),
        2 => solutions::day2::solve(day),
        3 => solutions::day3::solve(day),
        4 => solutions::day4::solve(day),
        5 => solutions::day5::solve(day),
        6 => solutions::day6::solve(day),
        // ... add more days
        _ => println!("Day {} not implemented yet", day),
    }
}