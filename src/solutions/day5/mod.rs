use crate::utils;
use std::collections::{HashMap, HashSet};

fn part1(graph: &HashMap<i32, Vec<i32>>, second_part_array: &Vec<Vec<i32>>) -> i32 {
    let mut middle_sum = 0;
    for elements in second_part_array {
        if elements.windows(2).all(|w| graph.get(&w[0]).map_or(false, |n| n.contains(&w[1]))) {
            middle_sum += elements[elements.len() / 2]; // Add the middle element value
        }
    }
    middle_sum
}


fn part2(graph: &HashMap<i32, Vec<i32>>, second_part_array: &Vec<Vec<i32>>) -> i32 {
    let mut corrected_lines = Vec::new();
    
    for elements in second_part_array {
        // Check if the line follows the graph rules
        let is_valid = elements.windows(2).all(|pair| {
            if let Some(neighbors) = graph.get(&pair[0]) {
                neighbors.contains(&pair[1])
            } else {
                false
            }
        });
        if !is_valid {
            corrected_lines.push(correct_order(&elements, &graph)); // Correct the line
        }
    }
    // corrected_lines

    let mut middle_sum = 0;
    for line in corrected_lines {
        middle_sum += line[line.len() / 2]; // Add the middle element value
    }
    middle_sum
}

// Correct the order of elements in a line based on the graph
fn correct_order(line: &Vec<i32>, graph: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    for &element in line {
        if !graph.contains_key(&element) {
            println!("Warning: Element {} is not defined in the graph!", element);
        }
    }
    // Helper function for DFS
    
    fn dfs(node: i32, graph: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>, result: &mut Vec<i32>) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                dfs(neighbor, graph, visited, result);
            }
        }
        result.push(node);
    }

    for &node in line {
        dfs(node, graph, &mut visited, &mut result);
    }

    // Reverse the result to get the correct topological order
    result.reverse();
    // Retain only the elements from the original line
    result.into_iter().filter(|x| line.contains(x)).collect()
}

fn get_graph(graph_input: &str) ->HashMap<i32, Vec<i32>>{
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

    // Build the graph from the input
    graph_input.lines().for_each(|line| {
        let mut parts = line.split('|').filter_map(|s| s.trim().parse::<i32>().ok());
        if let (Some(from), Some(to)) = (parts.next(), parts.next()) {
            graph.entry(from).or_insert_with(Vec::new).push(to);
        }
    });
    graph
}

fn get_string_array(lines_input: &str) -> Vec<Vec<i32>>{
    let mut second_part_array: Vec<Vec<i32>> = Vec::new();
    for line in lines_input.lines() {
        let elements: Vec<i32> = line.split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        second_part_array.push(elements);
    }
    second_part_array
}


pub fn solve(day: u32) {
    println!("Started Day{}!",day );

    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {
        let parts: Vec<&str> = contents.split("\n\n").collect();
        let graph = get_graph(parts[0]);
        let second_part_array = get_string_array(parts[1]);

        let part1 = part1(&graph,&second_part_array) ; 
        let part2 = part2(&graph,&second_part_array);
        println!("Part1:  {}", part1);
        println!("Part2:  {:?}", part2);
    }
}