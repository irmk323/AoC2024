use std::fs;

// type Grid = Vec<Vec<char>>;

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
// fn parse_grid(input: &str) -> Grid {
//     input
//         .lines()
//         .map(|line| line.chars().collect())
//         .collect()
// }


