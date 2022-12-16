#![feature(iter_array_chunks)]
use anyhow::Result;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn find_common_char(s: &str) -> char {
    let half = s.len() / 2;
    let mut st = HashSet::new();
    for ch in s.chars().take(half) {
        st.insert(ch);
    }

    for ch in s.chars().rev().take(half) {
        if st.contains(&ch) {
            return ch;
        }
    }
    unreachable!()
}

fn to_priority(ch: char) -> i32 {
    return match ch {
        'a'..='z' => ch as u8 - b'a' + 1,
        _ => ch as u8 - b'A' + 27,
    } as i32;
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let chs = first.chars().collect::<HashSet<_>>();

            to_priority(second.chars().find(|p| chs.contains(p)).unwrap())
        })
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|arr| {
            to_priority(
                *arr.into_iter()
                    .map(|s| s.chars().collect::<HashSet<_>>())
                    .reduce(|a, b| a.intersection(&b).copied().collect())
                    .unwrap()
                    .iter()
                    .exactly_one()
                    .unwrap(),
            )
        })
        .sum::<i32>()
}

fn main() -> Result<()> {
    let content = read_to_string("data/day3.txt")?;
    println!("part1:{}", part1(&content));
    println!("part2:{}", part2(&content));
    Ok(())
}
