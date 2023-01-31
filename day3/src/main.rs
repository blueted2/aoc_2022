#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::env;
use std::fs;

use std::char;
use std::hash::Hash;

trait Priority {
    fn priority(&self) -> Option<u8>;
}

impl Priority for char {
    fn priority(&self) -> Option<u8> {
        let char_u8 = *self as u8;
        
        const a_u8: u8  = 'a' as u8;
        const z_u8: u8  = 'z' as u8;
        const A_u8: u8  = 'A' as u8;
        const Z_u8: u8  = 'Z' as u8;

        match char_u8 {
            a_u8..=z_u8 => Some(char_u8 - a_u8 + 1),
            A_u8..=Z_u8 => Some(char_u8 - A_u8 + 27),
            _ => None
        }

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let r: u32 = contents
        .lines()
        .map(|line| {
            
            let len = line.len();

            let first: HashSet<char> = line[..len/2].chars().collect();
            let second: HashSet<char> = line[len/2..].chars().collect();
            
            let common_char = first.intersection(&second).next().unwrap();

            common_char.priority().unwrap() as u32
        }).sum();
    
    // dbg!(r);
        
    let r2: u32 = contents
        .lines().array_chunks::<3>()
        .map(|group| {
            let mut sets: [HashSet<char>; 3] = group.map(|line| {
                HashSet::from_iter(line.chars())
            });

            let (first, others) = sets.split_at_mut(1);
            let first = &mut first[0];

            for other in others {
                first.retain(|e| other.contains(e));
            }

            first.iter().next().unwrap().priority().unwrap() as u32
            }).sum();

    dbg!(r2);
            
}