// use itertools::Itertools;
use std::collections::HashMap;

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
    Folder(HashMap<String, Path>),
}

impl Path {
    fn new_folder() -> Self {
        Self::Folder(HashMap::new())
    }

    fn add_file(&mut self, name: &str, size: usize) {
        match self {
            Self::File(_) => panic!("Cannot add inode to file {}", name),
            Self::Folder(contents) => contents.insert(name.to_owned(), Self::File(size)),
        };
    }

    fn add_folder(&mut self, name: &str) -> &mut Path {
        match self {
            Self::File(_) => panic!("Cannot add inode to file {}", name),
            Self::Folder(contents) => contents
                .entry(name.to_owned())
                .or_insert_with(Self::new_folder),
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
                    path.print_out(name, indent + 2);
                }
            }
        }
    }
}

fn main() {
    let mut root = Path::new_folder();

    let a = root.add_folder("a");
    a.add_file("1", 1);
    a.add_file("2", 2);

    let c = a.add_folder("c");
    c.add_file("5", 5);
    c.add_file("6", 6);
    c.add_file("7", 7);

    let b = root.add_folder("b");
    b.add_file("3", 3);
    b.add_file("4", 4);

    root.print_out("/", 0);
}
