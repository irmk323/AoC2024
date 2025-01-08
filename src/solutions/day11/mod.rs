use crate::utils;
use std::collections::HashMap;

/// 入力文字列をパースし、初期の石のカウントマップを作成する関数
fn parse_input(input: &str) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    for num_str in input.trim().split_whitespace() {
        if let Ok(num) = num_str.parse::<u64>() {
            *map.entry(num).or_insert(0) += 1;
        }
    }
    map
}


/// 数字の桁数を数える関数
fn count_digits(num: u64) -> usize {
    if num == 0 {
        return 1;
    }
    let mut count = 0;
    let mut n = num;
    while n > 0 {
        n /= 10;
        count +=1;
    }
    count
}

/// 数字を偶数桁で分割する関数
/// 左半分と右半分の数字を返す
fn split_even_digits(num: u64) -> (u64, u64) {
    let s = num.to_string();
    let len = s.len();
    let half = len / 2;
    let left = s[..half].parse::<u64>().unwrap_or(0);
    let right = s[half..].parse::<u64>().unwrap_or(0);
    (left, right)
}

/// Blinkを1回適用し、新しい石のカウントマップを返す関数
fn apply_blink(current_map: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_map = HashMap::new();

    for (&num, &count) in current_map.iter() {
        if num == 0 {
            // ルール1: 0 → 1
            *new_map.entry(1).or_insert(0) += count;
        } else {
            let digits = count_digits(num);
            if digits % 2 == 0 {
                // ルール2: 偶数桁
                let (left, right) = split_even_digits(num);
                *new_map.entry(left).or_insert(0) += count;
                *new_map.entry(right).or_insert(0) += count;
            } else {
                // ルール3: その他
                let new_num = num * 2024;
                *new_map.entry(new_num).or_insert(0) += count;
            }
        }
    }

    new_map
}

/// Blinkを指定された回数適用し、最終的な石のカウントマップを返す関数
fn apply_blinks(initial_map: &HashMap<u64, u64>, blinks: u32) -> HashMap<u64, u64> {
    let mut current_map = initial_map.clone();
    for _ in 0..blinks {
        current_map = apply_blink(&current_map);
    }
    current_map
}

/// 最終的な石の総数を計算する関数
fn total_stones(final_map: &HashMap<u64, u64>) -> u64 {
    final_map.values().sum()
}
/// メインのソルバーファンクション
pub fn solve(day: u32) {
    println!("Started Day{}!", day);
    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {
    
    let initial_map = parse_input(&contents);

    // 25回のBlinkを適用する
    let final_map = apply_blinks(&initial_map, 25);
    // println!("Final Stones after 25 blinks: {:?}", final_map);

    // 最終的な石の総数を計算
    let total = total_stones(&final_map);
    println!("Part1: Total number of stones after 25 blinks: {}", total);

    // 75回のBlinkを適用する
    let final_map_75 = apply_blinks(&initial_map, 75);
    // println!("Final Stones after 75 blinks: {:?}", final_map_75);

    // 最終的な石の総数を計算
    let total = total_stones(&final_map_75);
    println!("Part2: Total number of stones after 75 blinks: {}", total); 
    }
}