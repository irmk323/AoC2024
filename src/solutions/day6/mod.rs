use crate::utils;
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// 右に90度回転
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East  => Direction::South,
            Direction::South => Direction::West,
            Direction::West  => Direction::North,
        }
    }

    /// 現在の方向に基づいて移動のデルタを返す
    fn move_delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East  => (0, 1),
            Direction::South => (1, 0),
            Direction::West  => (0, -1),
        }
    }
}

fn part1(grid: &Grid) -> usize {
    // 開始位置と方向の特定
    let (start_pos, start_dir) = match find_start(&grid) {
        Some((pos, dir)) => (pos, dir),
        None => {
            println!("開始位置 '^' がグリッド内に見つかりません。");
            return 0;
        }
    };

    // 移動のシミュレーション
    let max_steps = 90000; // 安全策として最大ステップ数を設定
    let  (visited, _reached_edge)  = simulate_movement(&grid, start_pos, start_dir, max_steps);
    visited.len()
}




fn part2(grid: &Grid) -> usize {
    // 開始位置と方向の特定
    let (start_pos, start_dir) = match find_start(&grid) {
        Some((pos, dir)) => (pos, dir),
        None => {
            println!("開始位置 '^' がグリッド内に見つかりません。");
            return 0;
        }
    };

    let problematic_positions = find_problematic_positions(&grid, start_pos, start_dir);
    problematic_positions.len()
}


fn find_start(grid: &Grid) -> Option<((usize, usize), Direction)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let direction = match cell {
                '^' => Direction::North,
                '>' => Direction::East,
                'v' => Direction::South,
                '<' => Direction::West,
                _   => continue,
            };
            return Some(((i, j), direction));
        }
    }
    None
}


fn is_obstacle(grid: &Grid, pos: (usize, usize)) -> bool {
    let (x, y) = pos;
    if x >= grid.len() || y >= grid[0].len() {
        return true; // グリッド外は障害物とみなす
    }
    match grid[x][y] {
        '.' | '^' | '>' | 'v' | '<' => false, // 移動可能なセル
        _ => true, // その他は障害物
    }
}

/// ガードがグリッドのエッジに到達したかどうかを判定する関数
fn is_on_edge(grid: &Grid, pos: (usize, usize)) -> bool {
    let (x, y) = pos;
    x == 0 || y == 0 || x == grid.len() - 1 || y == grid[0].len() - 1
}

fn simulate_movement(
    grid: &Grid,
    start_pos: (usize, usize),
    start_dir: Direction,
    max_steps: usize,
) -> (HashSet<(usize, usize)>, bool){
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;
    visited.insert(current_pos);

    let mut loop_detected = false;

    for _step in 0..max_steps {
        // 現在の方向に基づいて次の位置を計算
        let (dx, dy) = current_dir.move_delta();
        let new_x = current_pos.0 as isize + dx;
        let new_y = current_pos.1 as isize + dy;

        // 新しい位置がグリッド外に出る場合、終了
        if new_x < 0 || new_x >= grid.len() as isize || new_y < 0 || new_y >= grid[0].len() as isize {
            println!("ガードがグリッドのエッジに到達しました: ({}, {})", current_pos.0, current_pos.1);
            loop_detected = false;
            break;
        }

        let new_pos = (new_x as usize, new_y as usize);

        if is_obstacle(grid, new_pos) {
            // 障害物がある場合は右に90度回転
            current_dir = current_dir.turn_right();
        } else {
            // 障害物がない場合は前進
            current_pos = new_pos;
            visited.insert(current_pos);

            // 移動後の位置がエッジなら終了
            if is_on_edge(grid, current_pos) {
                // println!("ガードがグリッドのエッジに到達しました: ({}, {})", current_pos.0, current_pos.1);
                loop_detected = false;
                break;
            }
            // Check for loop by position and direction
            let state = current_pos;
            if !visited.insert(state) {
                loop_detected = true; // Revisited state, infinite loop detected
            }            
        }
    }

    (visited, loop_detected)
}

/// List all positions where placing an obstacle causes an infinite loop
fn find_problematic_positions(grid: &Grid, start_pos: (usize, usize), start_dir: Direction) -> Vec<(usize, usize)> {
    let mut problematic = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // Skip if the cell is already an obstacle or the start position
            if is_obstacle(grid, (i, j)) || (i, j) == start_pos {
                continue;
            }

            // Create a modified grid with an obstacle at (i, j)
            let mut modified_grid = grid.clone();
            modified_grid[i][j] = '#';

            // Simulate movement
            let _max_steps = 10000;
            let (_visited, loop_detected)= simulate_movement(&modified_grid, start_pos, start_dir,_max_steps);
            if loop_detected{
                // If infinite loop detected, record the position
                problematic.push((i, j));
            }
        }
    }

    problematic
}

/// グリッドを表示する関数（オプション）
// fn display_grid(grid: &Grid) {
//     for row in grid {
//         let line: String = row.iter().collect();
//         println!("{}", line);
//     }
// }

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn solve(day: u32) {
    println!("Started Day{}!",day );

    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {
        let grid =  parse_grid(&contents);
        let part1 = part1(&grid) ; 
        let part2 = part2(&grid);
        println!("Part1:  {}", part1);
        println!("Part2:  {:?}", part2);
    }
}