use anyhow::Result;
use std::fs::read_to_string;

fn part1() -> Result<i32> {
    let content = read_to_string("data/day2.txt")?;
    let mut res = 0;
    for line in content.lines() {
        let mut arr = line.split_whitespace();
        let (oppo, sf) = (arr.next().unwrap(), arr.next().unwrap());

        match (oppo, sf) {
            ("A", "X") => res += 4,
            ("A", "Y") => res += 8,
            ("A", "Z") => res += 3,
            ("B", "X") => res += 1,
            ("B", "Y") => res += 5,
            ("B", "Z") => res += 9,
            ("C", "X") => res += 7,
            ("C", "Y") => res += 2,
            _ => res += 6,
        }
    }
    Ok(res)
}

fn part2() -> Result<i32> {
    let content = read_to_string("data/day2.txt")?;
    let mut res = 0;
    for line in content.lines() {
        let mut arr = line.split_whitespace();
        let (oppo, sf) = (arr.next().unwrap(), arr.next().unwrap());

        match (oppo, sf) {
            ("A", "X") => res += 3,
            ("A", "Y") => res += 4,
            ("A", "Z") => res += 8,
            ("B", "X") => res += 1,
            ("B", "Y") => res += 5,
            ("B", "Z") => res += 9,
            ("C", "X") => res += 2,
            ("C", "Y") => res += 6,
            _ => res += 7,
        }
    }
    Ok(res)
}

fn main() -> Result<()> {
    println!("part1:{}", part1()?);
    println!("part2:{}", part2()?);
    Ok(())
}
