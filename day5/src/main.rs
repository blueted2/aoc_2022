#![feature(iter_array_chunks)]

use std::{env, fs, str::FromStr, fmt::Display};

#[derive(Debug)]
struct Crate {
    value: char,
}

#[derive(Debug)]
struct CrateStacks {
    stacks: Vec<Vec<Crate>>,
}

#[derive(Debug)]
struct CrateMove {
    from: usize,
    to: usize,
    quantity: usize,
}

impl CrateStacks {
    fn move_crate(&mut self, from: usize, to: usize) {
        let c = self.stacks.get_mut(from).unwrap().pop().unwrap();

        println!("Move {} from {} to {}", c.value, from, to);
        self.stacks.get_mut(to).unwrap().push(c);
    }

    fn move_crates(&mut self, crate_move: CrateMove) {
        for _ in 0..crate_move.quantity {
            self.move_crate(crate_move.from, crate_move.to);
        }
    }
    
    fn move_crates_chunk(&mut self, crate_move: CrateMove) {
        let from = crate_move.from;
        let to = crate_move.to;
        let quantity = crate_move.quantity;

        let mut from_stack = &mut self.stacks[from];
        
        let chunk = &mut from_stack.split_off(from_stack.len()-quantity);
        
        let to_stack = &mut self.stacks[to];
        to_stack.append(chunk);
        
    }
}

#[derive(Debug)]
struct CrateStacksFromStrErr;

impl FromStr for CrateStacks {
    type Err = CrateStacksFromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // determine number of stacks
        let nb_stacks = s.lines().next().unwrap().len() / 4 + 1;

        // prepare the stack vectors
        let mut stacks: Vec<Vec<Crate>> = vec![];
        for _ in 0..nb_stacks {
            stacks.push(vec![]);
        }

        for line in s.lines().rev().skip(1) {
            let chars = line.chars().collect::<Vec<_>>();
            for stack_index in 0..stacks.len() {
                let char_index = 1 + 4 * stack_index;
                let char = chars[char_index];

                if char != ' ' {
                    println!("Added {char} to {stack_index}");
                    stacks[stack_index].push(Crate {value: char});
                }
            }
        }
        
        Ok(CrateStacks { stacks })
    }
}

impl FromStr for CrateMove {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words = line.split(" ").collect::<Vec<_>>();

        let quantity: usize = words[1].parse()?;
        let from: usize = words[3].parse()?;
        let to: usize = words[5].parse()?;

        let from = from - 1;
        let to = to - 1;

        Ok(CrateMove{ from, to, quantity })
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let (stack_lines, move_lines) = contents.split_once("\n\n").unwrap();


    let mut s = CrateStacks::from_str(&stack_lines).unwrap();

    let moves = move_lines
        .lines()
        .map(|line| CrateMove::from_str(line).unwrap());


    for m in moves {
        s.move_crates_chunk(m);
    }

    for stack in s.stacks {
        let last = stack.last().unwrap();
        print!("{}", last.value)
    }

}
