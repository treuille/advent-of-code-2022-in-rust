use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

enum Path {
    File(usize),
    Folder(HashMap<String, Rc<RefCell<Path>>>),
}

impl Path {
    fn new_folder() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::Folder(HashMap::new())))
    }

    fn add_file(&mut self, name: &str, size: usize) {
        match self {
            Self::File(_) => panic!("Cannot add inode to file {}", name),
            Self::Folder(children) => {
                children.insert(name.to_owned(), Rc::new(RefCell::new(Self::File(size))))
            }
        };
    }

    fn get_folder(&mut self, name: &str) -> Rc<RefCell<Path>> {
        match self {
            Self::File(_) => panic!("Cannot add inode to file {}", name),
            Self::Folder(children) => (*children)
                .entry(name.to_owned())
                .or_insert_with(Self::new_folder)
                .clone(),
        }
    }

    fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> Rc<RefCell<Self>> {
        let root = Self::new_folder();
        let mut cwd = vec![root.clone()];
        let mut lines = lines.peekable();
        while let Some(line) = lines.next() {
            let mut tokens = line.split(' ');
            match (tokens.next(), tokens.next(), tokens.next()) {
                (Some("$"), Some("cd"), Some("/")) => cwd = vec![root.clone()],
                (Some("$"), Some("cd"), Some("..")) => {
                    cwd.pop();
                }
                (Some("$"), Some("cd"), Some(name)) => {
                    let folder = cwd.last().unwrap().as_ref().borrow_mut().get_folder(name);
                    cwd.push(folder);
                }
                (Some("$"), Some("ls"), None) => {
                    let mut cwd = cwd.last().unwrap().as_ref().borrow_mut();
                    while let Some(line) = lines.peek() {
                        let mut tokens = line.split(' ');
                        match (tokens.next(), tokens.next()) {
                            (Some("$"), _) => break,
                            (Some("dir"), _) => (),
                            (Some(size), Some(name)) => cwd.add_file(name, size.parse().unwrap()),
                            _ => panic!("Unexpected line: {}", line),
                        }
                        lines.next();
                    }
                }
                _ => panic!("Unexpected line: {}", line),
            }
        }
        root
    }

    /// Returns (total_size, total_size_list) where:
    /// total_size - the total size of this path
    /// total_size_list - total sizes of all subfolders, including the root
    fn total_sizes(&mut self) -> (usize, Vec<usize>) {
        match self {
            Path::File(size) => (*size, vec![]),
            Path::Folder(children) => {
                let mut total_size = 0;
                let mut total_size_list = Vec::new();

                for child in children.values() {
                    let mut child = child.deref().borrow_mut();
                    let (child_total_size, child_total_size_list) = child.total_sizes();
                    total_size += child_total_size;
                    total_size_list.extend(child_total_size_list);
                }

                total_size_list.push(total_size);
                (total_size, total_size_list)
            }
        }
    }
}

fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_07.txt");
    let input_lines = input.trim().lines();
    let root = Path::parse_input(input_lines);

    // Solve a
    let (total_size, total_size_list) = root.deref().borrow_mut().total_sizes();
    let answer_a: usize = total_size_list.iter().filter(|x| **x <= 100000).sum();
    println!("day 7a: {} (1350966)", answer_a);

    // Solve b
    let must_free = total_size - 40000000;
    let answer_b = total_size_list.iter().filter(|x| **x >= must_free).min();
    println!("day 7b: {} (6296435)", answer_b.unwrap());
}
