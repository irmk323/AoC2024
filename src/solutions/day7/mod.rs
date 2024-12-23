use std::error::Error;

use crate::utils;

#[derive(Debug)]
struct Entry {
    prefix: u128,
    values: Vec<u128>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn part1(content: &str) -> u128 {

    let entries = match parse_input(&content) {
        Ok(entries) => entries, // If successful, get the Vec<Entry>
        Err(_) => return 0, // Handle the error case (return 0 or some default value)
    };
    // Calculate the sum of matching prefixes
    let matching_sum = sum_matching_prefixes(&entries);
    matching_sum
}

fn parse_input(input: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &str) -> Result<Entry, Box<dyn Error>> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return Err(format!("Line '{}' is not properly formatted.", line).into());
    }
    let prefix: u128 = parts[0].trim().parse()?;
    let values: Vec<u128> = parts[1]
        .trim()
        .split_whitespace()
        .map(|num_str| num_str.parse())
        .collect::<Result<Vec<u128>, _>>()?;
    Ok(Entry { prefix, values })
}


/// Generates all possible operator sequences (`+` or `*`) for `n` operator positions.
fn generate_operator_sequences(n: usize) -> Vec<Vec<char>> {
    let total_combinations = 1 << n; // 2^n combinations
    let mut sequences = Vec::new();

    for mask in 0..total_combinations {
        let mut seq = Vec::with_capacity(n);
        for i in 0..n {
            // If the i-th bit is set, use '+', else use '*'
            if (mask & (1 << i)) != 0 {
                seq.push('+');
            } else {
                seq.push('*');
            }
        }
        sequences.push(seq);
    }

    sequences
}

/// Evaluates the expression based on the operator sequence, left-to-right.
fn evaluate_expression(values: &[u128], operators: &[char]) -> u128 {
    let mut result: u128 = values[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            '+' => result += values[i + 1],
            '*' => result *= values[i + 1],
            _ => panic!("Unsupported operator: {}", op),
        }
    }
    result
}

/// Checks if any operator sequence matches the prefix.
fn check_operator_sequences(prefix: u128, values: &[u128]) -> bool {
    if values.len() == 0 {
        return false;
    }
    if values.len() == 1 {
        return values[0] == prefix;
    }

    let n = values.len() - 1;
    let operator_sequences = generate_operator_sequences(n);

    for ops in operator_sequences {
        let result = evaluate_expression(values, &ops);
        if result == prefix {
            return true;
        }
    }

    false
}

/// Function to sum the prefixes where operator sequences match.
fn sum_matching_prefixes(entries: &[Entry]) -> u128 {
    let mut total_sum = 0;
    for entry in entries {
        if check_operator_sequences(entry.prefix, &entry.values) {
            total_sum += entry.prefix;
        }
    }
    total_sum
}

fn part2(content: &str) -> u128 {
    let entries = match parse_input(content) {
        Ok(entries) => entries, // If successful, get the Vec<Entry>
        Err(_) => return 0, // Handle the error case (return 0 or some default value)
    };
    let part2_result = sum_matching_prefixes_for_part2(&entries);
    part2_result
}

/// 条件を満たすエントリのプレフィックスを合計する関数
fn sum_matching_prefixes_for_part2(entries: &[Entry]) -> u128 {
    entries.iter()
        .filter(|entry| check_operator_sequences_part2(entry.prefix, &entry.values))
        .map(|entry| entry.prefix)
        .sum()
}

fn generate_operator_sequences_part2(n: usize) -> Vec<Vec<Operator>> {
    let total_combinations = 3usize.pow(n as u32); // 3^n combinations
    let mut sequences = Vec::with_capacity(total_combinations);
    
    for mask in 0..total_combinations {
        let mut seq = Vec::with_capacity(n);
        let mut temp = mask;
        for _ in 0..n {
            let op_index = temp % 3;
            let op = match op_index {
                0 => Operator::Add,
                1 => Operator::Multiply,
                2 => Operator::Concatenate,
                _ => unreachable!(),
            };
            seq.push(op);
            temp /= 3;
        }
        sequences.push(seq);
    }
    
    sequences
}

fn evaluate_expression_part2(values: &[u128], operators: &[Operator]) -> Option<u128> {
    if values.is_empty() {
        return None;
    }
    
    let mut result = values[0];
    
    for (i, &op) in operators.iter().enumerate() {
        if i + 1 >= values.len() {
            return None; // 演算子が値より多い場合は無効
        }
        let next = values[i + 1];
        match op {
            Operator::Add => {
                result += next;
            }
            Operator::Multiply => {
                result *= next;
            }
            Operator::Concatenate => {
                // Concatenation: result と next を結合
                let concatenated = result.to_string() + &next.to_string();
                match concatenated.parse::<u128>() {
                    Ok(val) => result = val,
                    Err(_) => return None, // パースエラー（オーバーフローなど）
                }
            }
        }
    }
    
    Some(result)
}


fn check_operator_sequences_part2(prefix: u128, values: &[u128]) -> bool {
    if values.len() == 0 {
        return false;
    }
    if values.len() == 1 {
        return values[0] == prefix;
    }
    
    let n = values.len() - 1;
    let operator_sequences = generate_operator_sequences_part2(n);
    
    for ops in operator_sequences {
        if let Some(result) = evaluate_expression_part2(values, &ops) {
            if result == prefix {
                return true;
            }
        }
    }
    
    false
}

pub fn solve(day: u32) {
    println!("Started Day{}!",day );

    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {
        let part1: u128 = part1(&contents) ; 
        let part2 = part2(&contents);
        println!("Part1:  {}", part1);
        println!("Part2:  {}", part2);
    }
}