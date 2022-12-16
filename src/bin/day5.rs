use anyhow::Result;
use itertools::Itertools;
use std::fs::read_to_string;

fn part1(mut stacks: Vec<Vec<char>>, move_str: &str) -> String {
    move_str
        .lines()
        .map(|line| {
            let arr = line
                .split(" ")
                .map(|v| v.parse::<usize>().unwrap_or(0))
                .collect::<Vec<_>>();
            (arr[1], arr[3], arr[5])
        })
        .for_each(|(num, st, end)| {
            let idx = stacks[st].len() - num;
            let mut v = stacks[st].drain(idx..).rev().collect();
            stacks[end].append(&mut v);
        });

    stacks
        .iter()
        .skip(1)
        .map(|stack| *stack.last().unwrap())
        .collect::<String>()
}

fn part2(mut stacks: Vec<Vec<char>>, move_str: &str) -> String {
    move_str
        .lines()
        .map(|line| {
            let arr = line
                .split(" ")
                .map(|v| v.parse::<usize>().unwrap_or(0))
                .collect::<Vec<_>>();
            (arr[1], arr[3], arr[5])
        })
        .for_each(|(num, st, end)| {
            let idx = stacks[st].len() - num;
            let mut v = stacks[st].drain(idx..).collect();
            stacks[end].append(&mut v);
        });
    stacks
        .iter()
        .skip(1)
        .map(|stack| *stack.last().unwrap())
        .collect::<String>()
}

fn main() -> Result<()> {
    let content = read_to_string("data/day5.txt")?;
    let (stack_str, move_str) = content.split("\n\n").collect_tuple().unwrap();
    let mut stacks = vec![vec![]; 10];

    stack_str.lines().rev().skip(1).for_each(|line| {
        line.bytes()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|&(_, b)| b >= b'A' && b <= b'Z')
            .for_each(|(idx, b)| stacks[idx + 1].push(b as char));
    });

    println!("part1: {}", part1(stacks.clone(), move_str));
    println!("part2: {}", part2(stacks.clone(), move_str));
    Ok(())
}
