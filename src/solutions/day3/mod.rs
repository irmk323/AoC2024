use regex::Regex;
use crate::utils;

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    
    // Find all matches and sum their products
    re.captures_iter(input)
        .map(|cap| {
            // Parse the two numbers and multiply them
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    let mut enabled = true; 
    let mut current_pos = 0;

    for cap in mul_re.find_iter(input) {
        let mul_start = cap.start();
        
        // mul命令の前にdon't()があるか確認
        let preceding_text = &input[current_pos..mul_start];
        if let Some(dont_pos) = preceding_text.rfind("don't()") {
            let do_pos = preceding_text[dont_pos..].rfind("do()");
            // don't()の後にdo()がない場合は無効
            enabled = do_pos.is_some();
        } else if let Some(_) = preceding_text.rfind("do()") {
            enabled = true;
        }

        if enabled {
            // 有効な場合のみ計算して加算
            let cap = mul_re.captures(cap.as_str()).unwrap();
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            sum += x * y;
        }

        current_pos = mul_start + 1;
    }

    sum
}
pub fn solve(day: u32) {
    println!("Started Day{}!",day );

    if let Ok(contents) = utils::read_file(&format!("src/solutions/day{}/input.txt", day)) {
        let result = part1(&contents); 
        println!("Part1: Sum of multiplications: {}", result);
        let result2 = part2(&contents); 
        println!("Part2: Sum of multiplications: {}", result2);
    }
}