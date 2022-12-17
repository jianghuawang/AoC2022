use anyhow::Result;
use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Range {
    pub start: i32,
    pub end: i32,
}

impl Range {
    pub fn contains(&self, rhs: Range) -> bool {
        if self.start <= rhs.start && self.end >= rhs.end {
            true
        } else {
            false
        }
    }
    pub fn overlap(&self, rhs: Range) -> bool {
        if self.start <= rhs.end && self.end >= rhs.start {
            true
        } else {
            false
        }
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        Ok(Self {
            start: start.parse::<i32>()?,
            end: end.parse::<i32>()?,
        })
    }
}

fn part1(input: &str) -> Result<i32> {
    Ok(input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let first = first.parse::<Range>().unwrap();
            let second = second.parse::<Range>().unwrap();
            (first.contains(second) || second.contains(first)) as i32
        })
        .sum::<i32>())
}

fn part2(input: &str) -> Result<i32> {
    Ok(input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let first = first.parse::<Range>().unwrap();
            let second = second.parse::<Range>().unwrap();
            first.overlap(second) as i32
        })
        .sum::<i32>())
}

fn main() -> Result<()> {
    let content = read_to_string("data/day4.txt")?;
    println!("part1:{}", part1(&content)?);
    println!("part2:{}", part2(&content)?);
    Ok(())
}
