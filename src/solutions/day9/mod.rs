use crate::utils;

/// ディスクを `Option<u128>` のベクターとして表現
/// `None` は空きスペース（`.`）、`Some(id)` はファイルID
type Disk = Vec<Option<u128>>;

/// ディスクマップ文字列を解析し、ディスクの表現を作成する関数
/// ファイルと空きスペースを交互に処理し、ファイルIDを0から順に割り当てる
fn parse_disk_map(disk_map: &str) -> Disk {
    let mut disk: Disk = Vec::new();
    let mut chars = disk_map.chars();
    let mut is_file = true; // 最初はファイル
    let mut file_id: u128 = 0; // ファイルIDは0から開始

    while let Some(c) = chars.next() {
        if !c.is_digit(10) {
            continue; // 数字以外はスキップ
        }
        let length = c.to_digit(10).unwrap() as usize;
        if is_file {
            for _ in 0..length {
                disk.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..length {
                disk.push(None); // 空きスペース
            }
        }
        is_file = !is_file; // ファイルと空きスペースを交互に
    }

    disk
}

/// ディスクを段階的にコンパクトする関数
/// 左から順に空きスペースを見つけ、右側から最も右にあるファイルブロックを一つずつ移動
fn compact_disk_step_by_step(mut disk: Disk) -> Disk {
    loop {
        // 左から最初の空きスペースを見つける
        let first_free = disk.iter().position(|&c| c.is_none());

        match first_free {
            Some(pos_free) => {
                // 空きスペースの右側で最も右にあるファイルブロックを見つける
                let file_pos = if pos_free + 1 < disk.len() {
                    disk[pos_free + 1..]
                        .iter()
                        .rposition(|&c| c.is_some()) // 右から最初のファイルブロック
                        .map(|pos| pos_free + 1 + pos)
                } else {
                    None
                };

                match file_pos {
                    Some(pos_file) => {
                        // ファイルブロックを空きスペースに移動
                        disk[pos_free] = disk[pos_file];
                        disk[pos_file] = None;
                    }
                    None => {
                        // 空きスペースの右側にファイルブロックが無い場合、完了
                        break;
                    }
                }
            }
            None => {
                // 空きスペースが無い場合、完了
                break;
            }
        }
    }
    disk
}


/// チェックサムを計算する関数
/// 各ファイルブロックの位置とファイルIDの積を合計
fn calculate_checksum(disk: &Disk) -> u128 {
    disk.iter()
        .enumerate()
        .filter_map(|(pos, &block)| {
            if let Some(id) = block {
                Some(pos as u128 * id)
            } else {
                None
            }
        })
        .sum()
}


fn part1(contents: &str) -> u128{

    let parsed_disk = parse_disk_map(contents);
    // println!("Initial Disk: {}", display_disk(&parsed_disk));

    // Compact the disk
    let compacted_disk = compact_disk_step_by_step(parsed_disk);
    // println!("Compacted Disk: {}", display_disk(&compacted_disk));

    // Calculate the checksum
    let checksum = calculate_checksum(&compacted_disk);
    println!("Filesystem Checksum: {}", checksum);
    checksum
}

// fn part2(grid: &Vec<Vec<char>>) -> usize {
 
// }


// /// ディスクを視覚的に表示する関数
// /// 空きスペースは '.'、ファイルブロックはファイルIDを表示
// fn display_disk(disk: &Disk) -> String {
//     disk.iter()
//         .map(|block| match block {
//             Some(id) => id.to_string(),
//             None => ".".to_string(),
//         })
//         .collect::<Vec<String>>()
//         .join("")
// }


pub fn solve(day: u32) {
    println!("Started Day{}!",day );
    if let Ok(contents) =  utils::read_file( &format!("src/solutions/day{}/input.txt", day)) {
        let part1 = part1(&contents) ; 
        // let part2 = part2(&grid);
        println!("Part1:  {:?}", part1);
        // println!("Part2:  {:?}", part2);
    }
}