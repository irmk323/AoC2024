use crate::utils;
use std::collections::{HashSet, VecDeque};

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
fn find_all_regions(grid: &Grid, is_part1: bool   ) -> Vec<Region> {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            if !visited[i][j] {
                let region;
                if is_part1{
                    region  = explore_region_bfs(i, j, grid, &mut visited);
                }else{
                    region  = explore_region_bfs_part2(i, j, grid, &mut visited);
                }
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

/// 指定されたセルの周囲エッジを収集する関数
fn collect_perimeter_edges(i: usize, j: usize, grid: &Grid) -> HashSet<(usize, usize, char)> {
    let target_char = grid[i][j];
    let mut edges = HashSet::new();

    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // 各方向をチェック
    // 上
    if i == 0 || grid[i - 1][j] != target_char {
        edges.insert((i, j, 'U')); // 'U'は上
    }
    // 下
    if i + 1 == rows || grid[i + 1][j] != target_char {
        edges.insert((i, j, 'D')); // 'D'は下
    }
    // 左
    if j == 0 || grid[i][j - 1] != target_char {
        edges.insert((i, j, 'L')); // 'L'は左
    }
    // 右
    if j + 1 == cols || grid[i][j + 1] != target_char {
        edges.insert((i, j, 'R')); // 'R'は右
    }

    edges
}

/// BF Sを用いて区域を探索し、面積と辺の数を計算する関数
fn explore_region_bfs_part2(i: usize, j: usize, grid: &Grid, visited: &mut Vec<Vec<bool>>) -> Region {
    let target_char = grid[i][j];
    let mut queue = VecDeque::new();
    queue.push_back((i, j));
    visited[i][j] = true;

    let mut area = 0;
    let mut perimeter_edges: HashSet<(usize, usize, char)> = HashSet::new();

    while let Some((x, y)) = queue.pop_front() {
        area += 1;

        // 周囲のエッジを収集
        let edges = collect_perimeter_edges(x, y, grid);
        for edge in edges {
            perimeter_edges.insert(edge);
        }

        // 隣接セルを探索
        for (adj_x, adj_y) in get_adjacent(x, y, grid) {
            if grid[adj_x][adj_y] == target_char && !visited[adj_x][adj_y] {
                visited[adj_x][adj_y] = true;
                queue.push_back((adj_x, adj_y));
            }
        }
    }

    // エッジをトレースして辺の数をカウント
    let sides = count_sides(&perimeter_edges);

    let price = area * sides;

    Region {
        character: target_char,
        area,
        perimeter: sides,
        price,
    }
}

/// 周囲のエッジを元に辺の数をカウントする関数
fn count_sides(perimeter_edges: &HashSet<(usize, usize, char)>) -> usize {
    // エッジを方向別に分ける
    // 'U' と 'D' は水平エッジ
    // 'L' と 'R' は垂直エッジ
    let mut u_edges: Vec<(usize, usize)> = Vec::new();
    let mut d_edges: Vec<(usize, usize)> = Vec::new();
    let mut l_edges: Vec<(usize, usize)> = Vec::new();
    let mut r_edges: Vec<(usize, usize)> = Vec::new();

    for &(x, y, dir) in perimeter_edges.iter() {
        match dir {
            'U' => u_edges.push((x, y)),
            'D' => d_edges.push((x, y)),
            'L' => l_edges.push((x, y)),
            'R' => r_edges.push((x, y)),
            _ => (),
        }
    }

    // 部分関数: 同一方向内で連続するエッジを1つの辺としてカウント
    fn count_direction_sides(edges: &mut Vec<(usize, usize)>, by_x_y: bool) -> usize {
        if by_x_y {
            // 'U' と 'D' は同じ方法で処理（行ごとにグループ化）
            edges.sort_by_key(|&(x, y)| (x, y));
            let mut count = 0;
            let mut prev_x = None;
            let mut prev_y = None;

            for &(x, y) in edges.iter() {
                if Some(x) != prev_x {
                    // 新しい行
                    count += 1;
                } else if let Some(prev_y_val) = prev_y {
                    if y != prev_y_val + 1 {
                        // 列が連続していなければ新しい辺
                        count += 1;
                    }
                }
                prev_x = Some(x);
                prev_y = Some(y);
            }
            count
        } else {
            // 'L' と 'R' は同じ方法で処理（列ごとにグループ化）
            edges.sort_by_key(|&(x, y)| (y, x));
            let mut count = 0;
            let mut prev_y = None;
            let mut prev_x = None;

            for &(x, y) in edges.iter() {
                if Some(y) != prev_y {
                    // 新しい列
                    count += 1;
                } else if let Some(prev_x_val) = prev_x {
                    if x != prev_x_val + 1 {
                        // 行が連続していなければ新しい辺
                        count += 1;
                    }
                }
                prev_y = Some(y);
                prev_x = Some(x);
            }
            count
        }
    }

    // 辺の数をカウント
    let mut u_sorted = u_edges.clone();
    let mut d_sorted = d_edges.clone();
    let mut l_sorted = l_edges.clone();
    let mut r_sorted = r_edges.clone();

    let u_sides = count_direction_sides(&mut u_sorted, true);
    let d_sides = count_direction_sides(&mut d_sorted, true);
    let l_sides = count_direction_sides(&mut l_sorted, false);
    let r_sides = count_direction_sides(&mut r_sorted, false);

    u_sides + d_sides + l_sides + r_sides
}


/// メインのソルバーファンクション
pub fn solve(day: u32) {
    println!("Started Day{}!", day);
    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {
    
    // グリッドのパース
    let grid = parse_grid(&contents);
    // println!("Input Grid:\n{}", display_grid(&grid));

    // 全区域の探索
    let regions = find_all_regions(&grid, true);
    // 総価格の計算
    let total_price = calculate_total_price(&regions);
    println!("Part 1 Total Price: {}", total_price);

        // 全区域の探索
        let regions_part2 = find_all_regions(&grid, false);
        // 総価格の計算
        let total_price_part2 = calculate_total_price(&regions_part2);
        println!("Part 2 Total Price: {}", total_price_part2);
    }
}