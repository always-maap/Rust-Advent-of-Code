use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Error reading input.txt")
        .trim()
        .to_string();

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);
}

fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|i| i.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let mut each_elf = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|i| i.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    each_elf.sort_by(|a, b| b.cmp(a));
    println!("{:?}", each_elf);
    each_elf.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 45000);
    }
}
