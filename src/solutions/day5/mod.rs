use crate::utils;
use std::collections::{HashMap, VecDeque};

fn part1(graph: &HashMap<i32, Vec<i32>>, second_part_array: &Vec<Vec<i32>>) -> i32 {
    let mut middle_sum = 0;
    for elements in second_part_array {
        if elements.windows(2).all(|w| graph.get(&w[0]).map_or(false, |n| n.contains(&w[1]))) {
            middle_sum += elements[elements.len() / 2]; // Add the middle element value
        }
    }
    middle_sum
}


fn topological_sort(arr: &Vec<i32>, graph: &HashMap<i32, Vec<i32>>) -> Option<Vec<i32>> {
    // Initialize in-degree for each node in the array
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    for &node in arr {
        in_degree.entry(node).or_insert(0);
    }

    // Compute in-degree based on the graph's edges
    for &node in arr {
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if arr.contains(&neighbor) {
                    *in_degree.entry(neighbor).or_insert(0) += 1;
                }
            }
        }
    }

    // Queue for nodes with in-degree 0
    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter_map(|(&node, &deg)| if deg == 0 { Some(node) } else { None })
        .collect();

    let mut sorted = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if arr.contains(&neighbor) {
                    if let Some(deg) = in_degree.get_mut(&neighbor) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }

    if sorted.len() == arr.len() {
        Some(sorted)
    } else {
        // Cycle detected or incomplete sorting
        None
    }
}

fn part2(graph: &HashMap<i32, Vec<i32>>, second_part_array: &Vec<Vec<i32>>) -> i32 {
    let mut middle_sum = 0;
    for arr in second_part_array {
        // Check if the array follows the graph's order
        let is_correct = arr.windows(2).all(|w| {
            graph
                .get(&w[0])
                .map_or(false, |n| n.contains(&w[1]))
        });

        if !is_correct {
            // Reorder the array using topological sort
            if let Some(sorted) = topological_sort(arr, graph) {
                // println!("Original: {:?} --> Reordered: {:?}", arr, sorted);
                let middle = sorted[sorted.len() / 2];
                middle_sum += middle;
            } else {
                println!("Failed to sort array (possible cycle or incomplete constraints): {:?}", arr);
            }
        }
    }
    middle_sum
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