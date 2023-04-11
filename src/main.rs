// NYT Digits solver
// By: mjheagle

use std::collections::BTreeSet;
use std::io::stdin;

type PuzzleNumber = i16;

struct Puzzle {
    numbers: BTreeSet<PuzzleNumber>,
    solution: PuzzleNumber,
}

fn input_number() -> Result<PuzzleNumber, &'static str> {
    // read input
    let mut line = String::new();
    let r = stdin().read_line(&mut line);

    // check for input error
    if r.is_err() {
        return Err("read error");
    }

    // check that number was provided and return it
    let num = line.trim().parse::<PuzzleNumber>();
    if let Ok(n) = num {
        if n > 0 {
            return Ok(n);
        } else {
            return Err("number must be > 0");
        }
    } else {
        return Err("number must be input");
    }
}

fn input_pool() -> BTreeSet<PuzzleNumber> {
    let mut set = BTreeSet::new();
    println!("Enter numbers in pool: ");

    while set.len() < 6 {
        let num = input_number();
        match num {
            Ok(n) => {
                set.insert(n);
            },
            Err(e) => {
                println!("Error: {e}. Try again.");
            }
        }
    }

    set
}

fn input_puzzle() -> Puzzle {
    // input solution
    println!("Enter solution: ");
    let mut sol = input_number();
    while let Err(e) = sol {
        println!("Error: {e}. Try again");
    }
    let solution = sol.unwrap();
    
    // input pool of numbers
    let pool = input_pool();

    // create struct and return
    println!("Puzzle created: {pool:?} => {solution}");
    Puzzle {
        numbers: pool,
        solution
    }
}

fn main() {
    input_puzzle();
}
