use std::{io::{self, BufRead}};

fn main() {
    let mut input = elves_in_stdin();

    println!("top elf: {}", input.iter().max().unwrap().total());

    input.sort();
    println!("top three:");
    let reverse: Vec<&Elf> = input.iter().rev().collect();
    for &elf in &reverse[0..=2] {
        println!("{}", elf.total());
    }

    let tutti = totals(reverse[0..=2].to_vec());
    print!("top three total: {}", tutti.iter().sum::<isize>())
}

fn totals(elves: Vec<&Elf>) -> Vec<isize> {
    elves.iter().map( |elf| elf.total()).collect()
}

#[derive(Eq)]
struct Elf {
    calories: Vec<isize>
}

impl Elf {
    fn new() -> Elf {
        Elf { calories: [].to_vec() }
    }

    fn total(&self) -> isize {
        self.calories.iter().sum()
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total().cmp(&other.total())
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.total() == other.total()
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.total().cmp(&other.total()))
    }
}

fn elves_in_stdin() -> Vec<Elf> {
    let mut input = Vec::new();
    let mut current_elf = Elf::new();

    for bar in io::stdin().lock().lines() {
        match bar {
            Ok(input_line) => {
                if input_line.is_empty() {
                    input.push(current_elf);
                    current_elf = Elf::new();
                } else {
                    let calories:isize = input_line.parse().unwrap();
                    current_elf.calories.push(calories);
                }
            },
            Err(_) => {
                println!("Aww shiiieeet");
            },
        }
    }
    if !current_elf.calories.is_empty() {
        input.push(current_elf);
    }
    return input
}
