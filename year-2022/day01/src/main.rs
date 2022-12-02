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
