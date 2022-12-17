use std::fs::read_to_string;

use anyhow::Result;

fn part1(input: &str) -> usize {
    let mut marked = vec![vec![false; 99]; 99];
    let mut cols = vec![b'0' - 1; 99];
    let lines = input.lines().collect::<Vec<_>>();
    for (i, &line) in lines.iter().enumerate() {
        let mut max = b'0' - 1;
        for (j, b) in line.bytes().enumerate() {
            if b > max || b > cols[j] {
                marked[i][j] = true;
            }
            max = max.max(b);
            cols[j] = cols[j].max(b);
        }
    }

    let mut cols = vec![b'0' - 1; 99];
    for (i, &line) in lines.iter().enumerate().rev() {
        let mut max = b'0' - 1;
        for (j, b) in line.bytes().enumerate().rev() {
            if b > max || b > cols[j] {
                marked[i][j] = true;
            }
            max = max.max(b);
            cols[j] = cols[j].max(b);
        }
    }
    marked.into_iter().flatten().filter(|&b| b).count()
}

fn part2(input: &str) -> i32 {
    let mut scores = vec![vec![1; 99]; 99];
    let mut cols: Vec<Vec<(u8, i32)>> = vec![vec![]; 99];
    let lines = input.lines().collect::<Vec<_>>();
    for (i, &line) in lines.iter().enumerate() {
        let mut row: Vec<(u8, i32)> = vec![];
        for (j, b) in line.bytes().enumerate() {
            let mut left_score = 0;
            while !row.is_empty() && row.last().unwrap().0 < b {
                let last = row.pop().unwrap().1;
                left_score += last;
            }
            row.push((b, left_score + 1));
            left_score += if row.len() == 1 { 0 } else { 1 };

            let mut top_score = 0;
            while !cols[j].is_empty() && cols[j].last().unwrap().0 < b {
                let last = cols[j].pop().unwrap().1;
                top_score += last;
            }
            cols[j].push((b, top_score + 1));
            top_score += if cols[j].len() == 1 { 0 } else { 1 };

            scores[i][j] = left_score * top_score;
        }
    }

    let mut cols: Vec<Vec<(u8, i32)>> = vec![vec![]; 99];
    for (i, &line) in lines.iter().enumerate().rev() {
        let mut row: Vec<(u8, i32)> = vec![];
        for (j, b) in line.bytes().enumerate().rev() {
            let mut right_score = 0;
            while !row.is_empty() && row.last().unwrap().0 < b {
                let last = row.pop().unwrap().1;
                right_score += last;
            }
            row.push((b, right_score + 1));
            right_score += if row.len() == 1 { 0 } else { 1 };

            let mut bottom_score = 0;
            while !cols[j].is_empty() && cols[j].last().unwrap().0 < b {
                let last = cols[j].pop().unwrap().1;
                bottom_score += last;
            }
            cols[j].push((b, bottom_score + 1));

            bottom_score += if cols[j].len() == 1 { 0 } else { 1 };
            scores[i][j] *= right_score * bottom_score;
        }
    }
    scores.into_iter().flatten().max().unwrap()
}

fn main() -> Result<()> {
    let content = read_to_string("data/day8.txt")?;

    println!("part1: {}", part1(&content));
    println!("part2: {}", part2(&content));
    Ok(())
}
