use anyhow::Result;
use itertools::Itertools;
use std::fs::read_to_string;

fn part1(input: &str) -> i32 {

    input 
        .split("\n\n")
        .map(|multilines| {
            multilines
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    input 
        .split("\n\n")
        .map(|multilines| {
            multilines
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<i32>()
}

fn main() -> Result<()> {
    let content = read_to_string("data/day1.txt")?;
    println!("part1:{}", part1(&content));
    println!("part2:{}", part2(&content));
    Ok(())
}
