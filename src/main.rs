// NYT Digits solver
// By: mjheagle

use std::collections::BTreeSet;
use std::fmt;
use std::io::stdin;

type PuzzleNumber = i16;

/// Digits puzzle object
struct Puzzle {
    /// Pool of numbers used to create the target solution
    numbers: BTreeSet<PuzzleNumber>,
    /// Target solution
    solution: PuzzleNumber,
}
/// Define how to display puzzle object
impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Puzzle: {:?} => {}", self.numbers, self.solution)
    }
}
impl Puzzle {
    /// Create a new Puzzle object
    ///
    /// # Arguments
    ///
    /// * `solution` - the target number
    /// * `nums` - an array of 6 numbers used to create the solution
    pub fn new(solution: PuzzleNumber, nums: &[PuzzleNumber; 6]) -> Self {
        let numbers = BTreeSet::from(*nums);
        Puzzle { numbers, solution }
    }
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

fn input_pool() -> [PuzzleNumber; 6] {
    let mut set = [0; 6];
    let mut index = 0;
    println!("Enter numbers in pool: ");

    while index < 6 {
        let num = input_number();
        match num {
            Ok(n) => {
                set[index] = n;
                index += 1;
            }
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
    let p = Puzzle::new(solution, &pool);
    println!("Created {p}");
    p
}

fn main() {
    input_puzzle();
}
