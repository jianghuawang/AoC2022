use anyhow::Result;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::read_to_string;

fn part1() -> Result<i32> {
    let content = read_to_string("data/day1.txt")?;

    let mut sum = 0;
    let mut max_sum = 0;
    for line in content.lines() {
        if line.len() == 0 {
            max_sum = max_sum.max(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>()?;
        }
    }
    Ok(max_sum)
}

fn part2() -> Result<i32> {
    let content = read_to_string("data/day1.txt")?;

    let mut sum = 0;
    let mut mh = BinaryHeap::with_capacity(4);


    for line in content.lines() {
        println!("{}", line);
        if line.len() == 0 {
            mh.push(Reverse(sum));
            if mh.len() > 3 {
                mh.pop();
            }
            sum = 0;
        } else {
            sum += line.parse::<i32>()?;
        }
    }

    Ok(mh.into_iter().fold(0, |acc, Reverse(val)| acc + val))
}

fn main() -> Result<()> {
    println!("part1: {}", part1()?);
    println!("part2: {}", part2()?);
    Ok(())
}
