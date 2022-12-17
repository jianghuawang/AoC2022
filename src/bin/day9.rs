use std::{collections::HashSet, fs::read_to_string, str::FromStr};

use anyhow::Result;

fn to_unit(mut pos: i32) -> i32 {
    pos += if pos > 0 { 1 } else { -1 };

    pos / 2
}

struct Dir(i32, i32);

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Dir(-1, 0),
            "R" => Dir(0, 1),
            "L" => Dir(0, -1),
            _ => Dir(1, 0),
        })
    }
}

fn simulate(input: &str, len: usize) -> Result<usize> {
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut knots = vec![[0; 2]; len];

    let mut walk = |Dir(dx, dy), steps: i32| {
        for _ in 0..steps {
            knots[0][0] += dx;
            knots[0][1] += dy;
            for i in 1..knots.len() {
                let diffy = knots[i - 1][1] - knots[i][1];
                let diffx = knots[i - 1][0] - knots[i][0];
                if diffx.abs() > 1 || diffy.abs() > 1 {
                    knots[i][0] += to_unit(diffx);
                    knots[i][1] += to_unit(diffy);
                }
            }
            visited.insert((knots[len - 1][0], knots[len - 1][1]));
        }
    };

    for line in input.lines() {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse::<i32>()?;
        let dir = dir.parse::<Dir>()?;
        walk(dir, steps);
    }
    Ok(visited.len())
}


fn main() -> Result<()> {
    let content = read_to_string("data/day9.txt")?;

    println!("part1: {}", simulate(&content, 2)?);
    println!("part1: {}", simulate(&content, 10)?);
    Ok(())
}
