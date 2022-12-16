use anyhow::Result;
use std::fs::read_to_string;

fn to_score(s: u8) -> i32 {
    match s {
        b'A' => 0,
        b'B' => 1,
        b'C' => 2,
        b'X' => 0,
        b'Y' => 1,
        _ => 2,
    }
}

fn part1_res(oppo: i32, sf: i32) -> i32 {
    sf + 1 + (sf - oppo + 4) % 3 * 3
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| part1_res(to_score(line.as_bytes()[0]), to_score(line.as_bytes()[2])))
        .sum::<i32>()
}

fn part2_res(oppo: i32, res: i32) -> i32 {
    res * 3
        + 1
        + match res {
            0 => (oppo + 2) % 3,
            1 => oppo,
            _ => (oppo + 4) % 3,
        }
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| part2_res(to_score(line.as_bytes()[0]), to_score(line.as_bytes()[2])))
        .sum::<i32>()
}

fn main() -> Result<()> {
    let content = read_to_string("data/day2.txt")?;
    println!("part1:{}", part1(&content));
    println!("part2:{}", part2(&content));
    Ok(())
}
