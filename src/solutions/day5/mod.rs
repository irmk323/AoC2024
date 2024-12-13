use crate::utils;
use std::collections::HashMap;

fn part1(graph_input: &str, order_check_input: &str) -> i32 {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

    // Build the graph from the input
    graph_input.lines().for_each(|line| {
        let mut parts = line.split('|').filter_map(|s| s.trim().parse::<i32>().ok());
        if let (Some(from), Some(to)) = (parts.next(), parts.next()) {
            graph.entry(from).or_insert_with(Vec::new).push(to);
        }
    });

    let mut middle_sum = 0;

    // Process each order check line
    for line in order_check_input.lines() {
        let elements: Vec<i32> = line.split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if elements.windows(2).all(|w| graph.get(&w[0]).map_or(false, |n| n.contains(&w[1]))) {
            middle_sum += elements[elements.len() / 2]; // Add the middle element value
        }
    }
    middle_sum
}


// fn part2(graph_input: &str, order_check_input: &str) -> usize {

// }


pub fn solve(day: u32) {
    println!("Started Day{}!",day );

    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {
        let parts: Vec<&str> = contents.split("\n\n").collect();
        let first_part = parts[0]; 
        let second_part = parts[1];
        let part1 = part1(&first_part,second_part) ; 
        // let part2 = part2(&grid);
        println!("Part1:  {}", part1);
        // println!("Part2:  {}", part2);
    }
}