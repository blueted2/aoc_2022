use std::{env, fs, ops::{RangeInclusive}};


trait RangeInclusiveExt<Idx> where Idx: PartialOrd<Idx> {
    fn from_str(s: &str) -> Option<RangeInclusive<Idx>>;

    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

type Assignment = RangeInclusive<u32>;

impl RangeInclusiveExt<u32> for Assignment {
    fn from_str(s: &str) -> Option<Assignment> {
        let bounds_vec: Vec<u32> = s.split('-').map(|c| c.parse().unwrap()).collect();

        let (start, end) = (bounds_vec[0], bounds_vec[1]);

        Some(start..=end)
    }

    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
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
            let mut assignments = line.split(',');
            let (first, second) = (assignments.next().unwrap(), assignments.next().unwrap());

            let (first, second) = (Assignment::from_str(first).unwrap(), Assignment::from_str(second).unwrap());

            if first.contains_range(&second) || second.contains_range(&first) {1} else {0}
        })
        .sum();

    let r2: u32 = contents
        .lines()
        .map(|line| {
            let mut assignments = line.split(',');
            let (first, second) = (assignments.next().unwrap(), assignments.next().unwrap());

            let (first, second) = (Assignment::from_str(first).unwrap(), Assignment::from_str(second).unwrap());

            if first.overlaps(&second) || second.overlaps(&first) {1} else {0}
        })
        .sum();

    dbg!(r2);
}
