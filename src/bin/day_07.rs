use std::borrow::BorrowMut;
use std::ops::Deref;
// use std::ops::DerefMut;
// use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const TEST_INPUT: &str = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

#[derive(Debug)]
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
            Self::Folder(contents) => {
                contents.insert(name.to_owned(), Rc::new(RefCell::new(Self::File(size))))
            }
        };
    }

    fn get_folder(&mut self, name: &str) -> Rc<RefCell<Path>> {
        match self {
            Self::File(_) => panic!("Cannot add inode to file {}", name),
            Self::Folder(contents) => (*contents)
                .borrow_mut()
                .entry(name.to_owned())
                .or_insert_with(Self::new_folder)
                .clone(),
        }
    }

    fn print_out(&self, name: &str, indent: usize) {
        for _ in 0..indent {
            print!(" ");
        }
        match self {
            Self::File(size) => println!("- {} ({})", name, size),
            Self::Folder(contents) => {
                println!("- {}", name);
                for (name, path) in contents.iter() {
                    (*path).borrow().print_out(name, indent + 2);
                }
            }
        }
    }

    fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> Rc<RefCell<Self>> {
        let root = Self::new_folder();
        let mut cwd = vec![root.clone()];
        let mut lines = lines.peekable();
        while let Some(line) = lines.next() {
            println!("line: {}", line);
            let mut tokens = line.split(' ');
            match (tokens.next(), tokens.next(), tokens.next()) {
                (Some("$"), Some("cd"), Some("/")) => cwd = vec![root.clone()],
                (Some("$"), Some("cd"), Some("..")) => {
                    cwd.pop();
                }
                (Some("$"), Some("cd"), Some(path)) => {
                    let folder = cwd.last().unwrap().as_ref().borrow_mut().get_folder(path);
                    cwd.push(folder);
                }
                (Some("$"), Some("ls"), None) => {
                    println!("** ls");
                    let mut cwd = cwd.last().unwrap().as_ref().borrow_mut();
                    while let Some(line) = lines.peek() {
                        println!("line: {}", line);
                        let mut tokens = line.split(' ');
                        match (tokens.next(), tokens.next()) {
                            (Some("$"), _) => break,
                            (Some("dir"), _) => (),
                            (Some(size), Some(path)) => cwd.add_file(path, size.parse().unwrap()),
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
            Path::Folder(contents) => {
                let mut total_size = 0;
                let mut total_size_list = Vec::new();

                for child in contents.values() {
                    let mut child = child.deref().borrow_mut();
                    let (child_total_size, child_total_size_list) = child.total_sizes();
                    total_size += child_total_size;
                    total_size_list.extend(child_total_size_list);
                }

                total_size_list.push(total_size);
                (total_size, total_size_list)
                // if total_size <= 100000 {
                //     print!(
                //         "found path of size {} moving total_size {} ->",
                //         total_size, size_sum
                //     );
                //     size_sum += total_size;
                //     println!("{}", size_sum);
                // }
                // (total_size, size_sum)
            }
        }
    }
}

fn main() {
    let input = include_str!("../../puzzle_inputs/day_07.txt")
        .trim()
        .lines();
    // let input = TEST_INPUT.trim().lines();
    let root = Path::parse_input(input);
    root.deref().borrow_mut().print_out("/", 0);
    let (total_size, size_sum) = solve_a(&mut root.deref().borrow_mut()); // borrow_mut());
    println!("total_size: {}", total_size);
    println!("size_sum: {}", size_sum);

    let (total_size, total_size_list) = root.deref().borrow_mut().total_sizes();
    println!("total_size: {}", total_size);
    println!("total_size_list: {:?}", total_size_list);
    println!(
        "answer: {}",
        total_size_list
            .iter()
            .filter(|x| **x <= 100000)
            .sum::<usize>()
    );

    // total_size - x >= 40000000
    // -x <= 40000000 - total_size
    // x >= total_size - 40000000

    assert!(total_size > 40000000, "Nothing to free");
    let must_free = total_size - 40000000;
    println!(
        "answer b: {}",
        total_size_list
            .iter()
            .filter(|x| **x >= must_free)
            .min()
            .unwrap()
    );
}

/// Returns the (total_size, size_sum) where
/// total_size - The total size of this path.
/// size_sum - The sum of all total_sizes < 100000 rooted here, includind path.  
fn solve_a(path: &mut Path) -> (usize, usize) {
    // let path = path.as_ref().borrow_mut();
    match path.deref() {
        Path::File(size) => (*size, 0),
        Path::Folder(contents) => {
            // HashMap<String, Rc<RefCell<Path>>>
            let mut total_size = 0;
            let mut size_sum = 0;
            for path in contents.values() {
                let path = &mut *path.deref().borrow_mut();
                let (child_total_size, child_size_sum) = solve_a(path);
                total_size += child_total_size;
                size_sum += child_size_sum;
            }
            if total_size <= 100000 {
                print!(
                    "found path of size {} moving total_size {} ->",
                    total_size, size_sum
                );
                size_sum += total_size;
                println!("{}", size_sum);
            }
            (total_size, size_sum)
        }
    }
}

// fn _main() {
//     let root = Path::new_folder();
//     {
//         let mut root = (*root).borrow_mut();

//         let a = root.get_folder("a");
//         {
//             let mut a = (*a).borrow_mut();
//             a.add_file("1", 1);
//             a.add_file("2", 2);

//             let c = a.get_folder("c");
//             {
//                 let mut c = (*c).borrow_mut();
//                 c.add_file("5", 5);
//                 c.add_file("6", 6);
//                 c.add_file("7", 7);
//             }

//             let b = a.get_folder("b");
//             {
//                 let mut b = (*b).borrow_mut();
//                 b.add_file("3", 3);
//                 b.add_file("4", 4);
//             }
//         }
//     }

//     (*root).borrow_mut().print_out("/", 0);
// }
