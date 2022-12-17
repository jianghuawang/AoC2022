use std::fs::read_to_string;

use anyhow::Result;
use itertools::Itertools;

fn part1(input: &str) -> usize {
    input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, arr)| arr.iter().all_unique())
        .unwrap()
        .0
        + 4
}

fn part2(input: &str) -> usize {
    input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, arr)| arr.iter().all_unique())
        .unwrap()
        .0
        + 14
}

fn main() -> Result<()> {
    let content = read_to_string("data/day6.txt")?;

    println!("part1: {}", part1(&content));
    println!("part2: {}", part2(&content));
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::part1;

    #[test]
    fn test_part1() {
        let arr = [
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let res: Vec<_> = arr.iter().map(|input| part1(input)).collect();
        assert_eq!(res, vec![5, 6, 10, 11]);
    }
}
