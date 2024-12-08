mod solutions {
    pub mod day1;
    pub mod day2;
    pub mod day3;
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
        // ... add more days
        _ => println!("Day {} not implemented yet", day),
    }
}