use crate::utils;
use std::collections::HashSet;

type Grid = Vec<Vec<u32>>;

/// 入力文字列をグリッドにパースする関数
fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect()
}

// /// グリッドを視覚的に表示する関数（デバッグ用）
// fn display_grid(grid: &Grid) -> String {
//     grid.iter()
//         .map(|row| {
//             row.iter()
//                 .map(|&num| num.to_string())
//                 .collect::<Vec<String>>()
//                 .join("")
//         })
//         .collect::<Vec<String>>()
//         .join("\n")
// }

/// グリッド内の数値の位置を特定する関数
fn find_positions(grid: &Grid, target: u32) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            if num == target {
                positions.push((i, j));
            }
        }
    }
    positions
}

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

/// 各'0'から到達可能な'9'の数を数える関数
fn count_reachable_nines(grid: &Grid) -> u32 {
    let zeros = find_positions(grid, 0);
    let nines = find_positions(grid, 9);
    let nines_set: HashSet<(usize, usize)> = nines.iter().cloned().collect();

    let mut total_count = 0;

    for &(start_i, start_j) in &zeros {
        // 到達可能な'9'を保持するセット
        let mut reachable_nines: HashSet<(usize, usize)> = HashSet::new();

        // 現在の探索レベルにあるセルの位置を保持
        let mut current_level: HashSet<(usize, usize)> = HashSet::new();
        current_level.insert((start_i, start_j));

        // 1から9まで順に探索
        for target_num in 1..=9 {
            let mut next_level: HashSet<(usize, usize)> = HashSet::new();

            for &(i, j) in &current_level {
                for &(adj_i, adj_j) in &get_adjacent(i, j, grid) {
                    if grid[adj_i][adj_j] == target_num {
                        next_level.insert((adj_i, adj_j));
                    }
                }
            }

            if target_num == 9 {
                // '9'に到達した位置を収集
                for pos in &next_level { // 参照を用いる
                    if nines_set.contains(pos) {
                        reachable_nines.insert(*pos); // 必要に応じてデリファレンス
                    }
                }
            }

            // 次の探索レベルに更新
            current_level = next_level;
        }

        // 到達可能な'9'の数をカウント
        total_count += reachable_nines.len() as u32;
    }

    total_count
}



/// 深さ優先探索（DFS）を使用して、`0`から`9`へのすべてのルートを探索し、カウントする関数
fn count_paths_from_zero_to_nine_part2(grid: &Grid, start: (usize, usize)) -> u32 {
    let mut count = 0;
    let rows = grid.len();
    let _cols = if rows > 0 { grid[0].len() } else { 0 };

    // DFS用のスタック: (現在の位置, 現在のターゲット数, 訪問済みセルのセット)
    let mut stack: Vec<(usize, usize, u32, HashSet<(usize, usize)>)> = Vec::new();

    // 初期状態: 開始位置、次に探す数字は1、訪問済みセットに開始位置を追加
    let mut initial_visited = HashSet::new();
    initial_visited.insert(start);
    stack.push((start.0, start.1, 1, initial_visited));

    while let Some((i, j, target_num, visited)) = stack.pop() {
        if target_num > 9 {
            // 9までたどり着いたルートをカウント
            count += 1;
            continue;
        }

        for &(adj_i, adj_j) in &get_adjacent(i, j, grid) {
            if grid[adj_i][adj_j] == target_num && !visited.contains(&(adj_i, adj_j)) {
                let mut new_visited = visited.clone();
                new_visited.insert((adj_i, adj_j));
                stack.push((adj_i, adj_j, target_num + 1, new_visited));
            }
        }
    }

    count
}


/// 各`0`から`9`へのルートの総数をカウントする関数
fn count_total_reachable_nines_part2(grid: &Grid) -> u32 {
    let zeros = find_positions(grid, 0);
    let mut total_count = 0;

    for &(i, j) in &zeros {
        total_count += count_paths_from_zero_to_nine_part2(grid, (i, j));
    }

    total_count
}

/// メインのソルバーファンクション
pub fn solve(day: u32) {
    println!("Started Day{}!", day);
    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {

    let grid = parse_grid(&contents);
    // println!("Input Grid:\n{}", display_grid(&grid));

    let total_reachable_nines = count_reachable_nines(&grid);
    println!("Total Reachable '9's: {}", total_reachable_nines);

    let total_reachable_nines_part2 = count_total_reachable_nines_part2(&grid);
    println!("Total Reachable '9's: {}", total_reachable_nines_part2);
    
    }
}