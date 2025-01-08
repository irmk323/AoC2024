use crate::utils;
use std::collections::VecDeque;

// this is 20 
// +-+-+-+-+-+
// |O O O O O|
// |O X O X O|
// |O O O O O|
// |O X O X O|
// |O O O O O|
// +-+-+-+-+-+

// +-+-+-+-+
// |A A A A|
// +-+-+-+-+     +-+
//               |D|
// +-+-+   +-+   +-+
// |B B|   |C|
// +   +   + +-+
// |B B|   |C C|
// +-+-+   +-+ +
//           |C|
// +-+-+-+   +-+
// |E E E|
// +-+-+-+

// src/solutions/day11
#[derive(Debug)]
#[allow(dead_code)]
struct Region {
    character: char,
    area: usize,
    perimeter: usize,
    price: usize,
}

/// グリッドを定義
type Grid = Vec<Vec<char>>;

/// 入力文字列をグリッドにパースする関数
fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

/// グリッドを視覚的に表示する関数（デバッグ用）
// fn display_grid(grid: &Grid) -> String {
//     grid.iter()
//         .map(|row| row.iter().collect::<String>())
//         .collect::<Vec<String>>()
//         .join("\n")
// }

/// セルの隣接セルを取得する関数（上下左右）
fn get_adjacent(i: usize, j: usize, grid: &Grid) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // 上
    if i > 0 {
        neighbors.push((i - 1, j));
    }
    // 下
    if i + 1 < rows {
        neighbors.push((i + 1, j));
    }
    // 左
    if j > 0 {
        neighbors.push((i, j - 1));
    }
    // 右
    if j + 1 < cols {
        neighbors.push((i, j + 1));
    }

    neighbors
}

/// BFSを用いて区域を探索し、面積と周囲長を計算する関数
fn explore_region_bfs(i: usize, j: usize, grid: &Grid, visited: &mut Vec<Vec<bool>>) -> Region {
    let target_char = grid[i][j];
    let mut queue = VecDeque::new();
    queue.push_back((i, j));
    visited[i][j] = true;

    let mut area = 0;
    let mut perimeter = 0;

    while let Some((x, y)) = queue.pop_front() {
        area += 1;

        // 各方向の隣接セルをチェック
        for (adj_x, adj_y) in get_adjacent(x, y, grid) {
            if grid[adj_x][adj_y] != target_char {
                // 異なる文字と接している場合、周囲長を1増加
                perimeter += 1;
            } else {
                // 同じ文字のセルが未訪問の場合、キューに追加
                if !visited[adj_x][adj_y] {
                    visited[adj_x][adj_y] = true;
                    queue.push_back((adj_x, adj_y));
                }
            }
        }

        // グリッドの境界と接している場合の周囲長をカウント
        let rows = grid.len();
        let cols = if rows > 0 { grid[0].len() } else { 0 };

        // 上方向
        if x == 0 {
            perimeter += 1;
        }
        // 下方向
        if x == rows - 1 {
            perimeter += 1;
        }
        // 左方向
        if y == 0 {
            perimeter += 1;
        }
        // 右方向
        if y == cols - 1 {
            perimeter += 1;
        }
    }

    let price = area * perimeter;

    Region {
        character: target_char,
        area,
        perimeter,
        price,
    }
}

/// 全区域を探索し、各区域の情報を収集する関数
fn find_all_regions(grid: &Grid) -> Vec<Region> {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            if !visited[i][j] {
                let region = explore_region_bfs(i, j, grid, &mut visited);
                regions.push(region);
            }
        }
    }

    regions
}

/// 総価格を計算する関数
fn calculate_total_price(regions: &Vec<Region>) -> usize {
    regions.iter().map(|r| r.price).sum()
}

/// メインのソルバーファンクション
pub fn solve(day: u32) {
    println!("Started Day{}!", day);
    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {
    
    // グリッドのパース
    let grid = parse_grid(&contents);
    // println!("Input Grid:\n{}", display_grid(&grid));

    // 全区域の探索
    let regions = find_all_regions(&grid);
    // 総価格の計算
    let total_price = calculate_total_price(&regions);
    println!("Part 1 Total Price: {}", total_price);
    }
}