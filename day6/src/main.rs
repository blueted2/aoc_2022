use std::{env, fs, collections::HashSet};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut gate = 0..14;

    while true {
        let chunk = contents.get(gate.clone()).unwrap().chars();

        let chunk: HashSet<char> = HashSet::from_iter(chunk);
        if chunk.len() == 14 {break;}

        gate.end += 1;
        gate.start += 1;
    }

    dbg!(gate.end);

}