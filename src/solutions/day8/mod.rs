use crate::utils;

use std::collections::{HashMap, HashSet};

fn compute_antinode_positions(p1: (usize, usize), p2: (usize, usize)) -> Vec<(usize, usize)> {
    let (r1, c1) = p1;
    let (r2, c2) = p2;

    // 反節点1: p3 = 2 * p2 - p1
    let p3_r = 2 * r2 as isize - r1 as isize;
    let p3_c = 2 * c2 as isize - c1 as isize;

    // 反節点2: p4 = 2 * p1 - p2
    let p4_r = 2 * r1 as isize - r2 as isize;
    let p4_c = 2 * c1 as isize - c2 as isize;

    let mut positions = Vec::new();

    // グリッドの範囲内かを確認
    if p3_r >= 0 && p3_c >= 0 {
        positions.push((p3_r as usize, p3_c as usize));
    }
    if p4_r >= 0 && p4_c >= 0 {
        positions.push((p4_r as usize, p4_c as usize));
    }

    positions
}

fn is_antenna(c: char) -> bool {
    c.is_alphanumeric() // アルファベットや数字をアンテナとみなす
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(grid: &Vec<Vec<char>>) -> usize{
    let num_rows = grid.len();
    if num_rows == 0 {
        println!("Empty grid.");
        return 0;
    }
    let num_cols = grid[0].len();
   // アンテナの収集と周波数によるグループ化
   let mut freq_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    
   for (r, row) in grid.iter().enumerate() {
       for (c, &ch) in row.iter().enumerate() {
           if is_antenna(ch) {
               freq_map.entry(ch).or_insert(Vec::new()).push((r, c));
           }
       }
   }
   
   // 反節点の位置を格納するハッシュセット
   let mut antinode_set: HashSet<(usize, usize)> = HashSet::new();
   
   // 各周波数グループでアンテナペアを処理
   for (_freq, antennas) in &freq_map {
       if antennas.len() < 2 {
           continue; // ペアが存在しない場合はスキップ
       }
       for i in 0..antennas.len() {
           for j in (i+1)..antennas.len() {
               let p1 = antennas[i];
               let p2 = antennas[j];
               
               // 反節点位置の計算
               let positions = compute_antinode_positions(p1, p2);
               
               for pos in positions {
                   // グリッドの範囲内か確認
                   if pos.0 < num_rows && pos.1 < num_cols {
                       antinode_set.insert(pos);
                   }
               }
           }
       }
   }
   
   // ユニークな反節点の総数
   let antinode_count = antinode_set.len();
   println!("Total unique antinode positions: {}", antinode_count);
   antinode_count
}

// fn part_2(left_vals: &mut Vec<i32>, right_vals: &mut Vec<i32>) {

// }


pub fn solve(day: u32) {
    println!("Started Day{}!",day );
    if let Ok(contents) =  utils::read_file( &format!("src/solutions/day{}/input.txt", day)) {
        let grid =  parse_grid(&contents);
        let part1 = part1(&grid) ; 
        println!("Part1:  {:?}", part1);
    }
}