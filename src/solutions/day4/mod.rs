use crate::utils;

fn part1(grid: &[Vec<char>]) -> usize {
    let directions = [
        (0, 1),  // Horizontal: Left to Right
        (0, -1), // Horizontal: Right to Left
        (1, 0),  // Vertical: Top to Bottom
        (-1, 0), // Vertical: Bottom to Top
        (1, 1),  // Diagonal: Top-Left to Bottom-Right
        (-1, -1),// Diagonal: Bottom-Right to Top-Left
        (1, -1), // Diagonal: Top-Right to Bottom-Left
        (-1, 1), // Diagonal: Bottom-Left to Top-Right
    ];

    let word = vec!['X', 'M', 'A', 'S'];
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            for (dr, dc) in &directions {
                let mut found = true;
                for (i, &ch) in word.iter().enumerate() {
                    let nr = r + dr * i as isize;
                    let nc = c + dc * i as isize;
                    if nr < 0 || nr >= rows || nc < 0 || nc >= cols || grid[nr as usize][nc as usize] != ch {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}



pub fn solve(day: u32) {
    println!("Started Day{}!",day );

    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {
        let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect()) // Convert each line to Vec<char>
        .collect();
        let part1 = part1(&grid); 
        let part2 = part2(&grid);
        println!("Part1:  {}", part1);
        println!("Part2:  {}", part2);
    }
}