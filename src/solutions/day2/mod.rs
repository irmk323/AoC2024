use crate::utils;

pub fn solve(day: u32) {
    println!("Started Day{}!",day );
    
    if let Ok(contents) = utils::read_file( &format!("src/solutions/day{}/input.txt", day)) {
        
        // Parse all lines once
        let all_numbers: Vec<Vec<i32>> = contents
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect()
            })
            .collect();
        
        // Use the parsed numbers for both counts
        let safe_count = all_numbers
            .iter()
            .filter(|numbers| is_safe_without_removal(numbers))
            .count();
            
        let with_removal_count = all_numbers
            .iter()
            .filter(|numbers| is_safe_sequence(numbers))
            .count();
        println!("Part1 {}", safe_count);
        println!("Part2 {}", with_removal_count);
    }
}

pub fn is_safe_sequence(numbers: &[i32]) -> bool {
    // First check if it's safe without removing any number
    if is_safe_without_removal(numbers) {
        return true;
    }

    // Try removing each number one at a time
    for skip_idx in 0..numbers.len() {
        let modified: Vec<i32> = numbers
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_idx)
            .map(|(_, &x)| x)
            .collect();
        
        if is_safe_without_removal(&modified) {
            return true;
        }
    }
    false
}

fn is_safe_without_removal(numbers: &[i32]) -> bool {

    let first_diff = numbers[1] - numbers[0];
    if first_diff == 0 {
        return false;
    }

    let is_increasing = first_diff > 0;
    
    for window in numbers.windows(2) {
        let diff = window[1] - window[0];
        if diff == 0 || diff.abs() > 3 || 
           (is_increasing && diff < 0) || 
           (!is_increasing && diff > 0) {
            return false;
        }
    }
    true
}

