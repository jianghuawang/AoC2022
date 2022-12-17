use std::fs::read_to_string;
use anyhow::Result;
use itertools::Itertools;

const DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

// solution inspired by @ageron
fn dis_to_tree_or_boarder(grid: &[Vec<u8>], mut x: i32, mut y: i32, dx: i32, dy: i32) -> i32 {
    let height = grid[x as usize][y as usize];
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    for step in 1.. {
        x += dx;
        y += dy;

        if x < 0 || y < 0 || x >= m || y >= n {
            return 1 - step;
        } else if grid[x as usize][y as usize] >= height {
            return step;
        }
    }
    unreachable!()
}

fn part1(grid: &[Vec<u8>]) -> usize {
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&(x, y)| {
            DIR.into_iter()
                .any(|(dx, dy)| dis_to_tree_or_boarder(grid, x as i32, y as i32, dx, dy) <= 0)
        })
        .count()
}

fn part2(grid: &[Vec<u8>]) -> i32 {
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .map(|(x, y)| {
            DIR.into_iter().fold(1, |prod, (dx, dy)| {
                prod * dis_to_tree_or_boarder(grid, x as i32, y as i32, dx, dy).abs()
            })
        })
        .max()
        .unwrap()
}

fn main() -> Result<()> {
    let content: Vec<_> = read_to_string("data/day8.txt")?
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();

    println!("part1: {}", part1(&content));
    println!("part2: {}", part2(&content));
    Ok(())
}
