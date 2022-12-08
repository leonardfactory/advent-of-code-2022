use std::collections::VecDeque;

use indextree::{Arena, NodeId};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum Line {
    ChangeDir(String),
    List,
    Output(String)
}

impl Line {
    pub fn new(line: &str) -> Line {
        let mut parts = line.split_whitespace();
        if parts.next() != Some("$") {
            return Line::Output(line.to_string());
        }

        match parts.next() {
            Some("cd") => Line::ChangeDir(parts.next().unwrap().to_string()),
            Some("ls") => Line::List,
            _ => panic!("Unknown command")
        }
    }
}

#[derive(Debug)]
pub enum FileSystemNode {
    Dir(String, usize),
    File(String, usize)
}

impl FileSystemNode {
    pub fn is_dir_named(&self, name: &str) -> bool {
        matches!(self, FileSystemNode::Dir(n, _) if n == name)
    }

    pub fn add_size(&mut self, size: usize) {
        match self {
            FileSystemNode::Dir(_, s) => *s += size,
            FileSystemNode::File(_, _) => panic!("Cannot add size to file")
        }
    }

    pub fn get_dir_size(&self) -> usize {
        match self {
            FileSystemNode::Dir(_, s) => *s,
            FileSystemNode::File(_, _) => 0
        }
    }
}

pub fn parse_tree(data: &str) -> (Arena<FileSystemNode>, NodeId) {
    let lines = data.lines().map(Line::new);
    let mut tree: Arena<FileSystemNode> = Arena::new();
    let root_id = tree.new_node(FileSystemNode::Dir("/".to_string(), 0));
    let mut current_id = root_id;

    for line in lines {
        match line {
            Line::ChangeDir(dir) if dir == "/" => {
                current_id = root_id;
            },
            Line::ChangeDir(dir) if dir == ".." => {
                current_id = tree.get(current_id).unwrap().parent().unwrap();
            },
            Line::ChangeDir(dir) => {
                let child = current_id
                    .children(&tree)
                    .find(|&n| tree.get(n).unwrap().get().is_dir_named(&dir));

                match child {
                    Some(child_id) => current_id = child_id,
                    None => {
                        let new_id = tree.new_node(FileSystemNode::Dir(dir, 0));
                        current_id.append(new_id, &mut tree);
                        current_id = new_id;
                    }
                }
            },
            Line::List => {},
            Line::Output(output) => {
                let (size_or_dir, name) = output.split_once(" ").unwrap();
                match size_or_dir {
                    "dir" => {
                        let new_id = tree.new_node(FileSystemNode::Dir(name.to_string(), 0));
                        current_id.append(new_id, &mut tree);
                    },
                    size => {
                        let size = size.parse::<usize>().unwrap();
                        let new_id = tree.new_node(FileSystemNode::File(name.to_string(), size));
                        current_id.append(new_id, &mut tree);

                        let ancestors = current_id.ancestors(&tree).collect_vec();
                        ancestors.iter().for_each(|&n| {
                            tree.get_mut(n).unwrap().get_mut().add_size(size);
                        });
                    }
                }
            }
        }
    }

    (tree, root_id)
}

fn print_filesystem(tree: &Arena<FileSystemNode>) {
    tree.iter().for_each(|n| {
        let node = n.get();
        let id = tree.get_node_id(n).unwrap();
        let depth = id.ancestors(tree).count();
        let indent = " ".repeat(depth * 2);
        match node {
            FileSystemNode::Dir(name, size) => println!("{}-{} ({})", indent, name, size),
            FileSystemNode::File(name, size) => println!("{} {} ({})", indent, name, size)
        }
    });
}

pub fn find_directories_by_total_size(input: &str) -> usize {
    let (tree, _root_id) = parse_tree(input);
    // print_filesystem(&tree);
    tree.iter()
        .filter(|n| n.get().get_dir_size() < 100000)
        .map(|n| n.get().get_dir_size()).sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_parse() {
        assert_eq!(Line::new("$ cd /"), Line::ChangeDir("/".to_string()));
        assert_eq!(Line::new("$ ls"), Line::List);
        assert_eq!(Line::new("ls"), Line::Output("ls".to_string()));
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(find_directories_by_total_size(input), 95437);
    }
}
