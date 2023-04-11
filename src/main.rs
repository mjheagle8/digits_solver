// NYT Digits solver
// By: mjheagle

use std::collections::BTreeSet;
use std::io::stdin;

fn input_number() -> Result<i16, &'static str> {
    // read input
    let mut line = String::new();
    let r = stdin().read_line(&mut line);

    // check for input error
    if r.is_err() {
        return Err("read error");
    }

    // check that number was provided and return it
    let num = line.trim().parse::<i16>();
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

fn input_pool() -> BTreeSet<i16> {
    let mut set = BTreeSet::new();
    println!("Enter numbers in pool: ");

    while set.len() < 6 {
        let num = input_number();
        match num {
            Ok(n) => {
                set.insert(n);
            },
            Err(e) => {
                println!("Error: {}. Try again.", e);
            }
        }
    }

    set
}

fn main() {
    let set = input_pool();
    println!("set: {set:?}");
}
