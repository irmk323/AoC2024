
use crate::utils;


/// 各ブロックの情報を保持する構造体
#[derive(Debug)]
struct Block {
    a_x: i128,   // Button AのX増分
    a_y: i128,   // Button AのY増分
    b_x: i128,   // Button BのX増分
    b_y: i128,   // Button BのY増分
    prize_x: i128, // PrizeのX目標値
    prize_y: i128, // PrizeのY目標値
}

/// 入力文字列を解析し、ブロックのリストを生成する関数
fn parse_blocks(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();

    // ブロックを空行で分割
    let raw_blocks = input.split("\n\n");
    
    for raw_block in raw_blocks {
        let mut a_x = 0;
        let mut a_y = 0;
        let mut b_x = 0;
        let mut b_y = 0;
        let mut prize_x = 0;
        let mut prize_y = 0;
        let mut valid = true;

        for line in raw_block.lines() {
            let line = line.trim();
            if line.starts_with("Button A:") {
                // Button Aを解析
                if let Some(rest) = line.strip_prefix("Button A:") {
                    let parts: Vec<&str> = rest.split(',').collect();
                    if parts.len() != 2 {
                        valid = false;
                        break;
                    }
                    for part in parts {
                        let part = part.trim();
                        if let Some(x_val) = part.strip_prefix("X+") {
                            a_x = match x_val.parse::<i128>() {
                                Ok(val) => val,
                                Err(_) => { valid = false; break; },
                            };
                        } else if let Some(y_val) = part.strip_prefix("Y+") {
                            a_y = match y_val.parse::<i128>() {
                                Ok(val) => val,
                                Err(_) => { valid = false; break; },
                            };
                        }
                    }
                } else {
                    valid = false;
                    break;
                }
            } else if line.starts_with("Button B:") {
                // Button Bを解析
                if let Some(rest) = line.strip_prefix("Button B:") {
                    let parts: Vec<&str> = rest.split(',').collect();
                    if parts.len() != 2 {
                        valid = false;
                        break;
                    }
                    for part in parts {
                        let part = part.trim();
                        if let Some(x_val) = part.strip_prefix("X+") {
                            b_x = match x_val.parse::<i128>() {
                                Ok(val) => val,
                                Err(_) => { valid = false; break; },
                            };
                        } else if let Some(y_val) = part.strip_prefix("Y+") {
                            b_y = match y_val.parse::<i128>() {
                                Ok(val) => val,
                                Err(_) => { valid = false; break; },
                            };
                        }
                    }
                } else {
                    valid = false;
                    break;
                }
            } else if line.starts_with("Prize:") {
                // Prizeを解析
                if let Some(rest) = line.strip_prefix("Prize:") {
                    let parts: Vec<&str> = rest.split(',').collect();
                    if parts.len() != 2 {
                        valid = false;
                        break;
                    }
                    for part in parts {
                        let part = part.trim();
                        if let Some(x_val) = part.strip_prefix("X=") {
                            prize_x = match x_val.parse::<i128>() {
                                Ok(val) => val,
                                Err(_) => { valid = false; break; },
                            };
                        } else if let Some(y_val) = part.strip_prefix("Y=") {
                            prize_y = match y_val.parse::<i128>() {
                                Ok(val) => val,
                                Err(_) => { valid = false; break; },
                            };
                        }
                    }
                } else {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            blocks.push(Block {
                a_x,
                a_y,
                b_x,
                b_y,
                prize_x,
                prize_y,
            });
        }
    }

    blocks
}


/// 各ブロックに対して最小トークン数を計算する関数
fn find_min_tokens(block: &Block) -> Option<i128> {
    let a1 = block.a_x;
    let a2 = block.a_y;
    let b1 = block.b_x;
    let b2 = block.b_y;
    let pX = block.prize_x;
    let pY = block.prize_y;

    let mut min_tokens = None;

    // xAは0からpX / a1まで（a1が正の場合）
    // xAが負の場合は反対方向も考慮が必要ですが、問題の文脈から正の整数と仮定
    if a1 <= 0 || b1 <= 0 || a2 <=0 || b2 <=0 || pX < 0 || pY <0 {
        return None; // 負の増分や目標値は未対応
    }

    let max_xA = pX / a1;
    // println!("max_xA {:?}", max_xA);

    for xA in 0..=max_xA {
        let remaining_X = pX - (xA * a1);
        if remaining_X % b1 != 0 {
            continue;
        }
        let xB = remaining_X / b1;
        if xB < 0 {
            continue;
        }

        let calculated_Y = xA * a2 + xB * b2;
        if calculated_Y == pY {
            let tokens = (3 * xA) + (1 * xB);
            if min_tokens.is_none() || tokens < min_tokens.unwrap() {
                min_tokens = Some(tokens);
            }
        }
    }

    min_tokens
}

/// 各ブロックに対して最小トークン数を計算する関数（パート2用）
fn find_min_tokens_part2(block: &Block) -> Option<u128> {
    let a1 = block.a_x;
    let a2 = block.a_y;
    let b1 = block.b_x;
    let b2 = block.b_y;
    let pX = block.prize_x;
    let pY = block.prize_y;

    let D = a1 * b2 - a2 * b1;

    if D == 0 {
        return None; // 解なし（行列が特異）
    }

    // xA = (pX * b2 - pY * b1) / D
    // xB = (a1 * pY - a2 * pX) / D

    // 両方が整数かどうかを確認
    if (pX * b2 - pY * b1) % D != 0 || (a1 * pY - a2 * pX) % D != 0 {
        return None; // 整数解なし
    }

    let xA = (pX * b2 - pY * b1) / D;
    let xB = (a1 * pY - a2 * pX) / D;

    // 非負整数であることを確認
    if xA < 0 || xB < 0 {
        return None; // 非負でない解
    }

    let tokens = (3 * xA) as u128 + (xB) as u128;

    Some(tokens)
}


/// メインのソルバーファンクション
pub fn solve(day: u32) {
    println!("Started Day{}!", day);
    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {
    
        let mut blocks = parse_blocks(&contents);
        // println!("Total Tokens: {:?}", blocks);
        let mut total_tokens: i128 = 0;
        
        for (index, block) in blocks.iter().enumerate() {
            match find_min_tokens(block) {
                Some(tokens) => {
                    // println!("Block {}: Minimum Tokens = {}", index + 1, tokens);
                    total_tokens += tokens;
                },
                None => {
                    // println!("Block {}: No exact combination found. Ignored.", index + 1);
                },
            }
        }

        println!("Total Tokens (Part 1): {}", total_tokens); 


        // PrizeのXとYをそれぞれ10000000000000増加
        let adjustment = 10_000_000_000_000i128;
        for block in blocks.iter_mut() {
            block.prize_x += adjustment;
            block.prize_y += adjustment;
        }
        let mut total_tokens_part2: u64 = 0;

        for (index, block) in blocks.iter().enumerate() {
            match find_min_tokens_part2(block) {
                Some(tokens) => {
                    // println!(
                    //     "Block {}: Minimum Tokens = {}",
                    //     index + 1,
                    //     tokens
                    // );
                    total_tokens_part2 += tokens as u64;
                }
                None => {
                    // println!(
                    //     "Block {}: No exact combination found. Ignored.",
                    //     index + 1
                    // );
                }
            }
        }
    
        println!("Total Tokens (Part 2): {}", total_tokens_part2);   
    }
}