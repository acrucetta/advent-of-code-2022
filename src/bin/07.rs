/*
Approach
- We need to create a folder structure based on the "cd" and the "ls" commands
    - The cd command will either move into a folder or move up a folder
    - The cd command can be either cd / or cd .. or cd folder_name
    - The ls command will list the contents of the current folder
- All commands in the iinput are preceeded by a $ sign
- We will create a tree structure that will represent the folder structure
    - Each node will have a name and a list of children, along with a parent
    - The root node will have a name of "root" and no parent
    - We will also have a size field for each node that contains the size of the folder or file
        - A directory will have the cumulative size of all its children

Methods
- parse the instructions
    - We will need to parse the cd and ls commands
    - We will need to parse the size of the file
- create the tree structure
    - We will need to create a node for each folder and file
    - We will need to add the node to the tree
- traverse the tree
    - We will need to traverse the tree and print out the contents of each folder
    - We will need to print out the size of each folder
- print the tree
- find_directories of at most N size
    - We will need to traverse the tree and find all directories that are at most N size
    - We will need to print out the name of the directory and its size
*/

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec;

// pub struct Node {
//     name: String,
//     size: u32,
//     children: Vec<Rc<RefCell<Node>>>,
//     parent: Option<Rc<RefCell<Node>>>,
//     node_type: FileType,
// }

// enum FileType {
//     Directory,
//     File,
// }

// impl Clone for FileType {
//     fn clone(&self) -> Self {
//         match self {
//             FileType::Directory => FileType::Directory,
//             FileType::File => FileType::File,
//         }
//     }
// }

// impl Node {
//     fn new(name: String, size: u32, node_type: FileType) -> Node {
//         Node {
//             name,
//             size,
//             children: vec![],
//             parent: None,
//             node_type: node_type,
//         }
//     }

//     fn clone(&self) -> Node {
//         Node {
//             name: self.name.clone(),
//             size: self.size,
//             children: self.children.iter().map(|child| child.clone()).collect(),
//             parent: None,
//             node_type: self.node_type.clone(),
//         }
//     }

//     fn add_child(&mut self, new_node: Rc<RefCell<Node>>) {
//         self.children.push(new_node);
//     }

//     fn get_size(&self) -> u32 {
//         match self.node_type {
//             FileType::Directory => {
//                 let mut size = 0;
//                 for child in &self.children {
//                     size += child.borrow().get_size();
//                 }
//                 size
//             }
//             FileType::File => self.size,
//         }
//     }

//     pub fn go_to_root(&self) -> Option<Rc<RefCell<Node>>> {
//         if let Some(parent) = &self.parent {
//             return parent.borrow().go_to_root();
//         } else {
//             return Some(Rc::clone(&self.parent));
//         }
//     }

//     fn calculate_directory_sizes(&self) -> HashMap<&String, u32> {
//         // We will need to traverse the tree and find all directories that are at most N size
//         // we will do a BFS traversal
//         let mut queue = Vec::new();
//         let mut directory_sizes = HashMap::new();
//         queue.push(self);

//         while !queue.is_empty() {
//             let node = queue.pop().unwrap();
//             let size = node.get_size();
//             directory_sizes.insert(&node.name, size);
//             for child in &node.children {
//                 queue.push(child);
//             }
//         }
//         directory_sizes
//     }
// }

// enum Command {
//     Cd(String),
//     Ls,
// }

// pub fn execute_command(input: &str, mut node: Rc<RefCell<Node>>) -> Node {
//     // We will need to parse the cd and ls commands and execute them
//     match input {
//         "/" => {
//             // We will need to move to the root node
//             return_node = node.get_root();
//         }
//         ".." => {
//             // We will need to move to the parent node
//             if let Some(parent) = node.parent {
//                 return_node = *parent;
//             }
//         }
//         _ => {
//             // We will need to move to the child node
//             for child in &node.children {
//                 if child.name == input {
//                     return_node = child.clone();
//                     break;
//                 }
//             }
//         }
//     }
//     return_node
// }

// pub fn part_one(input: &str) -> Option<u32> {
//     let root_node = Rc::new(RefCell::new(Node::new(
//         "root".to_string(),
//         0,
//         FileType::Directory,
//     )));
//     let mut current = Rc::clone(&root_node);

//     for line in input.lines() {
//         let mut command_parts: Vec<&str> = line.split_whitespace().collect();

//         if command_parts[0] == "$" {
//             match command_parts[1] {
//                 "cd" => {
//                     // We will need to execute the cd command
//                     let command = command_parts[2];
//                     let node = execute_command(command, root_node);
//                     root_node = node;
//                 }
//                 "ls" => {
//                     continue;
//                 }
//                 _ => panic!("Invalid command: {}", command_parts[1]),
//             }
//         } else {
//             // Add the file to the current node
//             if command_parts[0] == "dir" {
//                 let name = command_parts[1].to_string();
//                 let child_node = Rc::new(RefCell::new(Node::new(name, 0, FileType::Directory)));
//                 current.borrow_mut().children.push(Rc::clone(&child_node));

//                 let mut child_node = child_node.borrow_mut();
//                 child_node.parent = Some(Rc::clone(&current));
//             } else {
//                 let size = command_parts[0].parse::<u32>().unwrap();
//                 let name = command_parts[1].to_string();
//                 let child_node = Rc::new(RefCell::new(Node::new(name, size, FileType::Directory)));
//                 current.borrow_mut().children.push(Rc::clone(&child_node));

//                 let mut child_node = child_node.borrow_mut();
//                 child_node.parent = Some(Rc::clone(&current));
//             }
//         }
//     }
//     let directory_sizes = root_node.borrow().calculate_directory_sizes();
//     let max_size = 10000;
//     // Sum the sizes of all the directories that are at most 10000
//     let sum_size = directory_sizes
//         .iter()
//         .filter(|(_, size)| **size <= max_size)
//         .map(|(_, size)| size)
//         .sum();
//     Some(sum_size)
// }

enum FileType {
    Directory,
    File,
}

struct File {
    name: String,
    size: u32,
    file_type: FileType,
}

fn get_directory_sizes(directories: &HashMap<String, Vec<File>>, directory: &str) -> HashMap<String, u32> {
    let mut directory_sizes: HashMap<String, u32> = HashMap::new();
    let mut directory_size = 0;

    match directories.get(directory) {
        Some(files) => {
            for file in files {
                match file.file_type {
                    FileType::Directory => {
                        let child_directory_sizes = get_directory_sizes(directories, &file.name);
                        for (child_directory, child_directory_size) in child_directory_sizes {
                            let total_directory_size = directory_size + child_directory_size;
                            directory_sizes.insert(child_directory, total_directory_size);
                        }
                    }
                    _ => directory_size += file.size,
                }
            }
        }
        None => {}
    }

    directory_sizes.insert(directory.to_string(), directory_size);

    directory_sizes
}


pub fn part_one(input: &str) -> Option<u32> {
    // Create dictionary to store tuples of (file_name, size)
    let mut directories: HashMap<String, Vec<File>> = HashMap::new();

    let mut current_directory = String::from("/");
    directories.insert(current_directory.clone(), Vec::new());

    for line in input.lines() {
        let mut command_parts: Vec<&str> = line.split_whitespace().collect();

        if command_parts[0] == "$" {
            match command_parts[1] {
                "cd" => {
                    // We will need to execute the cd command
                    let command = command_parts[2];
                    match command {
                        "/" => {
                            current_directory = String::from("/");
                        }
                        ".." => {
                            // We won't need to do anything because we are just counting the files
                            // in the last directory
                        }
                        _ => {
                            // We will add this directory to the current directory
                            // If the directory already exists, we don't need to do anything
                            current_directory = command_parts[2].to_string();
                            if !directories.contains_key(&current_directory) {
                                directories.insert(current_directory.clone(), Vec::new());
                            }
                        }
                    }
                }
                "ls" => {
                    continue;
                }
                _ => panic!("Invalid command: {}", command_parts[1]),
            }
        } else {
            // Add the file to the current working directory
            if command_parts[0] == "dir" {
                let file = File {
                    name: command_parts[1].to_string(), 
                    size: 0,
                    file_type: FileType::Directory,
                };
                // Check if the file is in any File vector in the map
                let mut found = false;
                for (_, files) in directories.iter_mut() {
                    for f in files {
                        if f.name == file.name {
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    directories.get_mut(&current_directory).unwrap().push(file);
                }
            } else {
                let size = command_parts[0].parse::<u32>().unwrap();
                let file = File {
                    name: command_parts[1].to_string(), 
                    size,
                    file_type: FileType::File,
                };
                // Check if the file is in any File vector in the map
                let mut found = false;
                for (_, files) in directories.iter_mut() {
                    for f in files {
                        if f.name == file.name {
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    directories.get_mut(&current_directory).unwrap().push(file);
                }
            }
        }
    }
    let max_size = 100000;
    let directory_sizes = get_directory_sizes(&directories, "/");
    // Sum the sizes of all the directories that are at most 10000
    let sum_size = directory_sizes
        .iter()
        .filter(|(_, size)| **size <= max_size)
        .map(|(_, size)| size)
        .sum();
    Some(sum_size)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
