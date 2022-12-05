use std::{fs, ops::RangeInclusive};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Error reading the file")
        .trim()
        .to_string();

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);
}

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|range| {
                    range
                        .split("-")
                        .map(|n| n.parse().unwrap())
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .unwrap()
                })
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .filter(|(a, b)| a.contains_range(b) || b.contains_range(a))
        .count()
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 0);
    }
}
