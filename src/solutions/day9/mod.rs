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

/// ディスクを視覚的に表示する関数
/// 空きスペースは '.'、ファイルブロックはファイルIDを表示
// fn display_disk(disk: &Disk) -> String {
//     disk.iter()
//         .map(|block| match block {
//             Some(id) => id.to_string(),
//             None => ".".to_string(),
//         })
//         .collect::<Vec<String>>()
//         .join("")
// }

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

/// パート1: 各ブロックを左端の適切な空きスペースに一つずつ移動するコンパクション
fn part1(contents: &str) -> u128 {
    // ディスクマップを解析
    let parsed_disk = parse_disk_map(contents);
    // println!("Initial Disk (Part1): {}", display_disk(&parsed_disk));

    // コンパクト（ブロック単位の移動）
    let compacted_disk = compact_disk_step_by_step(parsed_disk);
    // println!("Compacted Disk (Part1): {}", display_disk(&compacted_disk));

    // チェックサムを計算
    let checksum = calculate_checksum(&compacted_disk);
    println!("Filesystem Checksum (Part1): {}", checksum);
    checksum
}

/// パート2: ファイル単位で左側の適切な空きスペースに移動するコンパクション
fn part2(contents: &str) -> u128 {
    // ディスクマップを解析
    let parsed_disk = parse_disk_map(contents);
    // println!("Initial Disk (Part2): {}", display_disk(&parsed_disk));

    // コンパクト（ファイル単位の移動）
    let compacted_disk = compact_disk_move_files_left(parsed_disk);
    // println!("Compacted Disk (Part2): {}", display_disk(&compacted_disk));

    // チェックサムを計算
    let checksum = calculate_checksum(&compacted_disk);
    println!("Filesystem Checksum (Part2): {}", checksum);
    checksum
}

/// パート1: 各ブロックを左端の適切な空きスペースに一つずつ移動するコンパクション
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

fn compact_disk_move_files_left(mut disk: Disk) -> Disk {
    // ファイルのリストを取得
    let mut files = list_files(&disk);

    // ファイルIDが高い順にソート
    files.sort_by(|a, b| b.0.cmp(&a.0));

    for (file_id, start, end) in files.iter() {
        let size = end - start;

        // 移動先の空きスペースを探す
        if let Some(target_pos) = find_leftmost_free_span(&disk, size) {
            if target_pos < *start {
                // 移動先の空きスペースにファイルが完全にフィットするか確認
                let can_fit = disk[target_pos..target_pos + size]
                    .iter()
                    .all(|&block| block.is_none());

                if can_fit {
                    // ファイルを移動
                    for pos in *start..*end {
                        disk[pos] = None;
                    }
                    for pos in 0..size {
                        disk[target_pos + pos] = Some(*file_id);
                    }
                }
                // 移動できない場合はそのまま
            }
            // 移動先が元の位置より右側の場合は移動しない
        }
        // 移動先の空きスペースがない場合はそのまま
    }

    disk
}


/// ファイルを特定し、ファイルIDとサイズをリスト化する関数
/// 戻り値は (file_id, start_pos, end_pos) のベクター
fn list_files(disk: &Disk) -> Vec<(u128, usize, usize)> {
    let mut files = Vec::new();
    let mut current_id: Option<u128> = None;
    let mut start_pos = 0;

    for (pos, &block) in disk.iter().enumerate() {
        match block {
            Some(id) => {
                if Some(id) != current_id {
                    // 新しいファイルの開始
                    if let Some(prev_id) = current_id {
                        // 前のファイルを保存
                        files.push((prev_id, start_pos, pos));
                    }
                    current_id = Some(id);
                    start_pos = pos;
                }
            }
            None => {
                if let Some(prev_id) = current_id {
                    // 現在のファイルを終了
                    files.push((prev_id, start_pos, pos));
                    current_id = None;
                }
            }
        }
    }

    // 最後のファイルが終了していない場合
    if let Some(prev_id) = current_id {
        files.push((prev_id, start_pos, disk.len()));
    }

    // 各ファイルのサイズを加えたリストを生成
    files.iter().map(|&(id, start, end)| (id, start, end)).collect()
}

/// 最も左にある適切な空きスペースを探す関数
fn find_leftmost_free_span(disk: &Disk, size: usize) -> Option<usize> {
    let mut consecutive = 0;
    for (pos, &block) in disk.iter().enumerate() {
        if block.is_none() {
            consecutive += 1;
            if consecutive == size {
                return Some(pos + 1 - size);
            }
        } else {
            consecutive = 0;
        }
    }
    None
}


/// ソルバーメイン関数
pub fn solve(day: u32) {
    println!("Started Day{}!", day);
    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {
        // パート1の処理
        let part1 = part1(&contents);
        println!("Part1 Checksum: {:?}", part1);

        // パート2の処理
        let part2 = part2(&contents);
        println!("Part2 Checksum: {:?}", part2);
    }
}