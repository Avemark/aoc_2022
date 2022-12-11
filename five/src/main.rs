use std::{io::BufRead};

use parser::{full_parse, Move};


mod parser;

fn main() {
    let input_blob = std::io::stdin().lock().lines().map( |line|  line.unwrap()).collect::<Vec<String>>().join("\n");

    match full_parse(&input_blob[..]) {
        Ok((_input, (mut stacks, moves))) => {
            // dbg!(&stacks);
            // dbg!(&moves);
            for Move { count, from, to} in moves {
                let len = stacks[from as usize - 1].len();
                let drained: Vec<&str> = stacks[from as usize -1]
                    .drain((len - count as usize)..)
                    .collect();
                for crater in drained {
                    stacks[to as usize - 1].push(crater);
                };
            }
            println!("");
            for mut stack in stacks {
                match stack.pop() {
                    Some(label) => print!("{}", label),
                    None => print!("[!] "),
                }
            }
        },
        Err(e) => println!("No good yo: {}", e),
    };
}

fn _apply_move_9000(stacks: &mut Vec<Vec<&str>>, crane_operation: Move) {
    for _ in 0..crane_operation.count {
        let element = stacks[crane_operation.from as usize - 1].pop().expect("Tried to take from empty stack");
        stacks[crane_operation.to as usize - 1].push(element);
    }
}
 