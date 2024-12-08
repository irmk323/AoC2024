#[path = "../utils.rs"]
mod utils;

use std::collections::HashMap;

fn main() {
    println!("started");
    
    if let Ok(contents) =  utils::read_file("input.txt") {
        
        if let Ok((mut left_vals, mut right_vals)) = process_content(&contents) {
            left_vals.sort();
            right_vals.sort();
            part_1(&mut left_vals, &mut right_vals);
            part_2(&mut left_vals, &mut right_vals);
        }
    }
}

fn part_1(left_vals: &mut Vec<i32>, right_vals: &mut Vec<i32>) {
    let diff_sum = calculate_differences(&left_vals, &right_vals);
    println!("Part1: Sum of differences: {}", diff_sum);
}

fn part_2(left_vals: &mut Vec<i32>, right_vals: &mut Vec<i32>) {
    let freq_sum = calculate_freq(&left_vals, &right_vals);
    println!("Part2: Sum of differences: {}", freq_sum);
}

fn calculate_freq(arr1: &Vec<i32>, arr2: &Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut map = HashMap::new();
    for &num in arr2 {
        *map.entry(num).or_insert(0) += 1;
    }
    for &val in arr1 {
        if let Some(&freq) = map.get(&val) {
            ans += &val * freq;
        }
    }
    ans
}

fn calculate_differences(arr1: &Vec<i32>, arr2: &Vec<i32>) -> i32 {
    let mut sum = 0;
    
    // Make sure arrays are of same length
    if arr1.len() == arr2.len() {
        for i in 0..arr1.len() {
            // Calculate absolute difference
            let diff = (arr1[i] - arr2[i]).abs();
            sum += diff;
        }
    }
    
    sum
}

fn process_content(contents: &str) -> Result<(Vec<i32>, Vec<i32>), &'static str> {
    let mut left_vals = Vec::new();
    let mut right_vals = Vec::new();

    for line in contents.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if numbers.len() == 2 {
            left_vals.push(numbers[0]);
            right_vals.push(numbers[1]);
        } else {
            return Err("Invalid format in line");
        }
    }

    Ok((left_vals, right_vals))
}