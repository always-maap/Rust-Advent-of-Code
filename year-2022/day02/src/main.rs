use std::{cmp::Ordering, fs, str::FromStr};

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

#[derive(PartialEq, Copy, Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(input: &str) -> Result<Choice, Self::Err> {
        match input {
            "X" | "A" => Ok(Choice::Rock),
            "Y" | "B" => Ok(Choice::Paper),
            "Z" | "C" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Choice::Scissors && other == &Choice::Rock {
            Some(Ordering::Less)
        } else if self == &Choice::Rock && other == &Choice::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

fn part1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line| {
            let game: Vec<Choice> = line
                .split(" ")
                .map(|c| c.parse::<Choice>().unwrap())
                .collect();

            let (elf, my) = (game[0], game[1]);

            match elf.partial_cmp(&my) {
                Some(Ordering::Greater) => 0 + my as u32,
                Some(Ordering::Equal) => 3 + my as u32,
                Some(Ordering::Less) => 6 + my as u32,
                None => panic!("you shoudn't be here!"),
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line| {
            let game = line.split(" ").collect::<Vec<_>>();

            let elf = game[0].parse::<Choice>().unwrap();

            let cond = game[1];

            match cond {
                "X" => {
                    // lose
                    let my = match elf {
                        Choice::Rock => Choice::Scissors,
                        Choice::Paper => Choice::Rock,
                        Choice::Scissors => Choice::Paper,
                    };
                    0 + my as u32
                }
                "Y" => {
                    // draw
                    let my = match elf {
                        Choice::Rock => Choice::Rock,
                        Choice::Paper => Choice::Paper,
                        Choice::Scissors => Choice::Scissors,
                    };
                    3 + my as u32
                }
                "Z" => {
                    // win
                    let my = match elf {
                        Choice::Rock => Choice::Paper,
                        Choice::Paper => Choice::Scissors,
                        Choice::Scissors => Choice::Rock,
                    };
                    6 + my as u32
                }
                _ => panic!("you shoudn't be here!"),
            }
        })
        .sum()
}
