use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Error reading the file")
        .trim()
        .to_string();

    let p1 = part1(&input);
    println!("Part 1: {}", p1);
}

fn part1(input: &str) -> usize {
    let letters = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    input
        .split("\n")
        .map(|line| {
            let mid = line.len() / 2;
            let (first, second) = (&line[0..mid], &line[mid..]);
            let common = first.chars().find(|c| second.contains(*c)).unwrap();
            let score = letters.get(&common).unwrap();
            score
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 157);
    }
}
