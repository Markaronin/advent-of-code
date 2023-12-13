use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::{Itertools, PeekingNext};

enum DoF {
    Dir {
        contents: BTreeMap<String, Rc<RefCell<DoF>>>,
        parent: Option<Rc<RefCell<DoF>>>,
    },
    File {
        size: usize,
        parent: Option<Rc<RefCell<DoF>>>,
    },
}
impl DoF {
    pub fn size(&self) -> usize {
        match self {
            DoF::File { size, .. } => *size,
            DoF::Dir { contents, .. } => contents.values().map(|entry| entry.borrow().size()).sum(),
        }
    }

    pub fn traverse(val: Rc<RefCell<Self>>) -> Vec<Rc<RefCell<DoF>>> {
        match &*val.clone().borrow() {
            DoF::File { .. } => vec![val],
            DoF::Dir { contents, .. } => {
                let mut a = contents
                    .values()
                    .flat_map(|child| DoF::traverse(child.clone()))
                    .collect_vec();
                a.push(val);
                a
            }
        }
    }

    pub fn add_child(&mut self, name: String, child: Rc<RefCell<DoF>>) {
        match self {
            DoF::Dir { contents, .. } => {
                contents.insert(name, child);
            }
            DoF::File { .. } => panic!("Tried to add a child to a file"),
        }
    }

    pub fn parent(&self) -> Option<Rc<RefCell<DoF>>> {
        match self {
            DoF::Dir { parent, .. } => parent.clone(),
            DoF::File { parent, .. } => parent.clone(),
        }
    }

    pub fn get_child(&self, key: &str) -> Option<Rc<RefCell<DoF>>> {
        match self {
            DoF::Dir { contents, .. } => contents.get(key).cloned(),
            DoF::File { .. } => panic!("Tried to get child of a file"),
        }
    }
}

// TODO - get rid of names, use hashmap of name to thing

fn get_program_output(input_file: &str) -> (usize, usize) {
    // Skip the first entry, which is "cd /"
    let mut input = read_lines(input_file).into_iter().skip(1).peekable();

    let root = Rc::new(RefCell::new(DoF::Dir {
        contents: BTreeMap::new(),
        parent: None,
    }));
    let mut current_dir = root.clone();

    while let Some(next_command) = input.next() {
        let next_command_split = next_command.split_ascii_whitespace().collect_vec();
        if next_command_split[1] == "ls" {
            while let Some(next_file) = input.peeking_next(|n| !n.starts_with("$ ")) {
                let next_file_split = next_file.split_ascii_whitespace().collect_vec();
                let next_file_name = next_file_split[1].to_string();
                let next_file_parsed = match next_file_split[0] {
                    "dir" => DoF::Dir {
                        contents: BTreeMap::new(),
                        parent: Some(current_dir.clone()),
                    },
                    raw_size => DoF::File {
                        size: raw_size.parse().unwrap(),
                        parent: Some(current_dir.clone()),
                    },
                };
                current_dir
                    .borrow_mut()
                    .add_child(next_file_name, Rc::new(RefCell::new(next_file_parsed)));
            }
        } else if next_command_split[1] == "cd" {
            current_dir = match next_command_split[2] {
                ".." => current_dir.borrow().parent(),
                val => current_dir.borrow().get_child(val),
            }
            .expect("Tried to cd into somewhere that didn't exist");
        } else {
            panic!("Unrecognized command")
        }
    }

    let all_files = DoF::traverse(root.clone());

    let result_1 = DoF::traverse(root.clone())
        .iter()
        .filter(|dof| match *dof.borrow() {
            DoF::File { .. } => false,
            DoF::Dir { .. } => true,
        })
        .filter_map(|dir| {
            let size = dir.borrow().size();
            if size <= 100000 {
                Some(size)
            } else {
                None
            }
        })
        .sum();

    let max_disk_space = 70000000;
    let unused_disk_space_needed = 30000000;
    let current_unused_disk_space = max_disk_space - root.clone().borrow().size();
    let remaining_needed_disk_space = unused_disk_space_needed - current_unused_disk_space;

    let result_2 = all_files
        .iter()
        .filter(|dof| match *dof.borrow() {
            DoF::File { .. } => false,
            DoF::Dir { .. } => true,
        })
        .filter_map(|dir| {
            let size = dir.borrow().size();
            if size >= remaining_needed_disk_space {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    (result_1, result_2)
}

base_aoc!(95437, 24933642);
