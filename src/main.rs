// NYT Digits solver
// By: mjheagle

use std::collections::{BTreeSet, VecDeque};
use std::fmt;
use std::io::stdin;

type PuzzleNumber = u32;

/// Operations possible
#[derive(Eq, PartialEq, Clone, Debug)]
enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Digits puzzle object
#[derive(Clone)]
struct Puzzle {
    /// Pool of numbers used to create the target solution
    numbers: BTreeSet<PuzzleNumber>,
    /// Target solution
    solution: PuzzleNumber,
    /// History of equations used to get to the current puzzle state
    history: Vec<PuzzleEquation>,
}
/// Define how to display puzzle object
impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Puzzle: {:?} => {}", self.numbers, self.solution)?;
        write!(f, "  [")?;
        for h in &self.history {
            write!(f, "{h};")?;
        }
        write!(f, "]")?;
        Ok(())
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
        let history = vec![];
        Puzzle {
            numbers,
            solution,
            history,
        }
    }

    /// Solve a Puzzle
    pub fn solve(&self) -> Option<Puzzle> {
        // initialize queue with base puzzle
        let mut queue = VecDeque::new();
        let copy = self.clone();
        queue.push_back(copy);
        let solution = self.solution;

        // calculate permutations of each item, adding new permutations to queue
        while let Some(p) = queue.pop_front() {
            // check if solution is in pool of numbers
            if p.numbers.contains(&solution) {
                println!("Solution found -> {p}");
                for history in &p.history {
                    println!("    {history}");
                }
                return Some(p);
            }
            // iterate through every pair of numbers
            for (index, &n0) in p.numbers.iter().enumerate() {
                for &n1 in p.numbers.iter().skip(index + 1) {
                    // create new pool, removing numbers to combine
                    let mut new_pool = p.numbers.clone();
                    new_pool.remove(&n1);
                    new_pool.remove(&n0);
                    // combine numbers with all types of Operations
                    let ops = [
                        Operations::Add,
                        Operations::Subtract,
                        Operations::Multiply,
                        Operations::Divide,
                    ];
                    for operation in ops {
                        // check numbers are divisible if division was performed
                        if operation == Operations::Divide && n1 % n0 != 0 {
                            continue;
                        }
                        // perform operation
                        let result = match operation {
                            Operations::Add => n1 + n0,
                            Operations::Subtract => n1 - n0,
                            Operations::Multiply => n1 * n0,
                            Operations::Divide => n1 / n0,
                        };
                        // add new number to pool
                        let mut numbers = new_pool.clone();
                        numbers.insert(result);
                        // create and append new history for this combination
                        let new_history = PuzzleEquation {
                            n0,
                            n1,
                            operation,
                            result,
                        };
                        let mut history = p.history.clone();
                        history.push(new_history);
                        // create new Puzzle object
                        let new_puzzle = Puzzle {
                            numbers,
                            solution,
                            history,
                        };
                        // add new Puzzle to queue
                        queue.push_back(new_puzzle);
                    }
                }
            }
        }
        // no solution found
        None
    }
}

/// Equation involving two numbers
#[derive(Clone, Debug)]
struct PuzzleEquation {
    /// First number in equation
    n0: PuzzleNumber,
    /// Second number in equation
    n1: PuzzleNumber,
    /// How the two numbers will be combined
    operation: Operations,
    /// Result of the equation
    result: PuzzleNumber,
}
impl fmt::Display for PuzzleEquation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match self.operation {
            Operations::Add         => "+",
            Operations::Subtract    => "-",
            Operations::Multiply    => "*",
            Operations::Divide      => "/",
        };
        write!(f, "{} {op} {} = {}", self.n1, self.n0, self.result)
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
    let sol = input_number();
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
    let p = input_puzzle();
    p.solve();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_solutions() {
        let p = Puzzle::new(66, &[1, 3, 5, 10, 4, 25]).solve();
        assert!(p.is_some());
        println!("{}", p.unwrap());
        let p = Puzzle::new(126, &[2, 10, 3, 15, 9, 25]).solve();
        assert!(p.is_some());
        println!("{}", p.unwrap());
        let p = Puzzle::new(234, &[3, 8, 4, 9, 6, 11]).solve();
        assert!(p.is_some());
        println!("{}", p.unwrap());
        let p = Puzzle::new(335, &[5, 11, 6, 15, 8, 20]).solve();
        assert!(p.is_some());
        println!("{}", p.unwrap());
        let p = Puzzle::new(476, &[5, 19, 7, 20, 11, 23]).solve();
        assert!(p.is_some());
        println!("{}", p.unwrap());
    }
}
