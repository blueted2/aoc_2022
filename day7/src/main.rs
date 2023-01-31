use slab_tree::*;

use core::panic;
use std::{fs, env};

enum FileType {
    File {size: i32},
    Directory,
}

struct FileItem {
    name: String,
    file_type: FileType,
}

impl FileItem {
    fn new_dir(name: &str) -> FileItem{
        FileItem { name: name.to_string(), file_type: FileType::Directory }
    }

    fn new_file(name: &str, size: i32) -> FileItem{
        FileItem {name: name.to_string(), file_type: FileType::File { size: size }}
    }
}


fn total_size_add_if_small(tree: &Tree<FileItem>, node_id: NodeId, acc: &mut i32) -> i32 {
    let file = tree.get(node_id).unwrap();

    match file.data().file_type {
        FileType::File { size } => size,
        FileType::Directory => {
            let folder_size = file
                .children()
                .map(|c| 
                    total_size_add_if_small(tree, c.node_id(), acc)
                )
                .sum();

            if folder_size < 100_000 {
                println!("found small folder: size={folder_size}");
                *acc += folder_size;
            }

            folder_size
        },
    }
}

fn total_size_track_smallest_larger_than(tree: &Tree<FileItem>, node_id: NodeId, acc: &mut i32, min: i32) -> i32 {
    let file = tree.get(node_id).unwrap();

    match file.data().file_type {
        FileType::File { size } => size,
        FileType::Directory => {
            let dir_size = file
                .children()
                .map(|c| 
                    total_size_track_smallest_larger_than(tree, c.node_id(), acc, min)
                )
                .sum();

            if dir_size > min && dir_size < *acc {
                *acc = dir_size;
            }
            dir_size
        },
    }
}




fn main() {
    let mut tree = TreeBuilder::new().with_root(FileItem::new_dir("/")).build();

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut current_dir_id = tree.root_id().unwrap();

    for line in contents.lines() {
        let words = line.split(" ").collect::<Vec<_>>();
        
        println!("-- : '{}' --", tree.get(current_dir_id).unwrap().data().name);

        match words[..] {
            ["$", "cd", "/"] => { 
                println!("move to root");
                current_dir_id = tree.root_id().unwrap();
            },
            ["$", "cd", ".."] => { 
                println!("move up");
                current_dir_id = tree.get(current_dir_id).unwrap().parent().unwrap().node_id();
            },
            ["$", "cd", dir] => {
                println!("move to '{dir}'");
                let child_id = tree
                    .get(current_dir_id)
                    .unwrap()
                    .children()
                    .find(|c| {
                        let file = c.data();
                        match file.file_type {
                            FileType::Directory if file.name == dir => true,
                            _ => false,
                        }
                    })
                    .unwrap()
                    .node_id();

                current_dir_id = child_id;
                
            },

            ["$", "ls"] => {println!("ls")},
            ["dir", name] => {
                let mut current_dir = tree.get_mut(current_dir_id).unwrap();
                println!("adding '{name}' to '{}'", current_dir.data().name);

                let new_dir = FileItem::new_dir(name);
                current_dir.append(new_dir);
            },
            [size, name] => {
                let mut current_dir = tree.get_mut(current_dir_id).unwrap();
                println!("adding '{name}', '{size}' to '{}'", current_dir.data().name);
                
                let new_file = FileItem::new_file(name, size.parse::<i32>().unwrap());
                current_dir.append(new_file);

            },
            _ => {panic!("unknown command")}
        };
    }

    let acc: &mut i32 = &mut 0;

    let total_size = total_size_add_if_small(&tree, tree.root_id().unwrap(), acc);

    dbg!(total_size);
    dbg!(acc);

    let acc: &mut i32 = &mut 30_000_000;

    let total_size = total_size_track_smallest_larger_than(&tree, tree.root_id().unwrap(), acc, 30_000_000 - (70_000_000 - total_size));

    dbg!(30_000_000 - (70_000_000 - total_size));

    dbg!(total_size);
    dbg!(acc);
    

}