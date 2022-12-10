use std::{collections::HashSet, io::{self, BufRead}};
#[allow(dead_code)]

const PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    // let uniques = uniques();
    // let sum = uniques.iter().fold(0,|acc, character| {
    //     // println!("{}: {}", character, priority(*character));
    //     acc + priority(*character) as usize
    // });
    // println!("Sum: {}", sum);
    let badges = badges();
    println!("Found {} badges.", badges.len());
    println!("sum: {}", badges.iter().map(|badge| priority(*badge) as usize ).sum::<usize>());
}

fn uniques() -> Vec<char> {
    let mut chars: Vec<char> = vec![];

    for result in io::stdin().lock().lines() {
        match result {
            Ok(line) => {
                let divider = line.len() / 2;
                let first = &line[..divider];
                let second = &line[divider..];
                chars.push(intersection(first, second));
            },
            Err(_) => panic!("failed to compile line"),
        }
    };

    return chars
}

fn badges() -> Vec<char> {
    let mut helpers: Vec<String> = vec![];
    let mut badges: Vec<char> = vec![];

    for result in io::stdin().lock().lines() {
        match result {
            Ok(line) => helpers.push(line),
            Err(_) => panic!("failed to compile line"),
        }
    };

    let mut helpers_it = helpers.iter();

    loop {
        let first_of_three = helpers_it.next();
        if first_of_three.is_none() {
            println!("done with {} helpers", helpers.len());
            break
        }
        let first_two = full_intersection(first_of_three.unwrap(), helpers_it.next().unwrap());
        let intersection = full_intersection(&first_two, helpers_it.next().unwrap());

        if intersection.len() != 1 {
            panic!("Failed to deal with '{}'", intersection)
        }
        badges.push(intersection.chars().next().unwrap());
        print!(".");
    }

    println!("!");
    return badges
}

fn full_intersection(first: &str, second: &str) -> String {
    let unique_a: HashSet<_> = first.chars().collect();
    let unique_b: HashSet<_> = second.chars().collect();
    return unique_a.intersection(&unique_b).collect();
}

fn intersection(first: &str, second: &str) -> char {
    let unique_a: HashSet<_> = first.chars().collect();
    let unique_b: HashSet<_> = second.chars().collect();
    let common = unique_a.intersection(&unique_b).into_iter().next();
    match common {
        Some(char) => char.clone(),
        None => panic!(
            "no common character found between {} and {}",
            first,
            second,
        ),
    }
}


fn priority(character: char) -> i8 {
    match PRIORITIES.find(character) {
        Some(n) => n as i8 + 1,
        None => 0,
    }
}
