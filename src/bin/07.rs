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

use std::collections::{HashMap, HashSet};

struct Node {
    name: String,
    size: u32,
    children: Vec<Node>,
    parent: Option<Box<Node>>,
    node_type: FileType,
}

enum FileType {
    Directory,
    File,
}

impl Node {
    fn new(name: String, size: u32) -> Node {
        Node {
            name,
            size,
            children: Vec::new(),
            parent: None,
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn add_parent(&mut self, parent: Node) {
        self.parent = Some(Box::new(parent));
    }

    fn get_size(&self) -> u32 {
        let mut size = self.size;
        for child in &self.children {
            size += child.get_size();
        }
        size
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

}

struct Tree {
    root: Node,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            root: Node::new("root".to_string(), 0),
        }
    }
}

enum Command {
    Cd(String),
    Ls,
}

pub fn execute_command(input: &str, mut node: Node) -> Node {
    // We will need to parse the cd and ls commands and execute them
    match input {
        "cd /" => {
            // We will need to move to the root node
        },
        "cd .." => {
            // We will need to move to the parent node
        },
        _ => {
            // We will need to move to the child node
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    
    let mut root_node = Node::new("/".to_string(), 0);
    let mut listing_files = false;

    for line in input.lines() {
        let mut command_parts : Vec<&str> = line.split_whitespace().collect();
        
        if command_parts[0] == "$" {
            listing_files = false; // We will reset the listing_files flag

            match command_parts[1] {
                "cd" => {
                    // We will need to execute the cd command
                    let command = command_parts[2];
                    let node = execute_command(command, root_node);
                    root_node = node;
                },
                "ls" => {
                    // We will need to execute the ls command
                    listing_files = true;
                }
            }
        } else {
            // Add the file to the current node
            if command_parts[0] == "dir" {
                let name = command_parts[1].to_string();
                let child_node = Node{name: name, size: 0, children: Vec::new(), parent: None, node_type: FileType::Directory};
                root_node.add_child(child_node);
            } else {
                let size = command_parts[0].parse::<u32>().unwrap();
                let name = command_parts[1].to_string();
                let child_node = Node{name: name, size: size, children: Vec::new(), parent: None, node_type: FileType::File};
                root_node.add_child(child_node);
            }
    }
    None
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
