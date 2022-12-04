use std::fs;

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

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 0);
    }
}
