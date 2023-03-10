
// 2:03 start
// 4:41 solved part 1. Fuck you, compiler.
// 8:47-10:23 studying Rust and cleaning up the code
// (day changed)
// 10:38 start part 2
// 11:06 solved part 2

#![allow(dead_code,unused_variables,unused_mut,unused_imports)]
use input_downloader;

use std::cell::RefCell;
use std::rc::Rc;


fn main() {
    let inputs = input_downloader::year(2022)
        .example("\
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
")
        .get_all();

    for input in inputs {
        Solver::new().solve(&input);
    }
}


#[derive(Debug)]
struct Dir {
    parent: Option<Rc<RefCell<Dir>>>,
    name: String,
    total_size: usize,
    files: Vec<Rc<RefCell<File>>>,
    subdirs: Vec<Rc<RefCell<Dir>>>
}

impl Dir {
    fn new(parent: Option<Rc<RefCell<Dir>>>, name: String) -> Dir {
        Dir {
            parent,
            name,
            total_size: 0,
            files: Vec::new(),
            subdirs: Vec::new()
        }
    }
}

#[derive(Debug)]
struct File {
    parent: Rc<RefCell<Dir>>,
    name: String,
    size: usize
}


struct Solver {
    total: usize,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            total: 0
        }
    }

    fn solve(mut self, input: &String) {
        let fs = Self::parse_input(input);

        self.du(Rc::clone(&fs));

        println!("total sizes = {}", self.total);

        let free_space = 70_000_000 - fs.borrow().total_size;
        let needed_space = 30_000_000 - free_space;
        println!("Need to find {needed_space} bytes to delete");

        let dir_sizes = Rc::new(RefCell::new(Vec::<usize>::new()));
        Self::list_dirs(fs, Rc::clone(&dir_sizes));
        let mut dir_sizes = dir_sizes.borrow_mut();
        dir_sizes.sort();
        println!("sizes: {dir_sizes:?}");
        if let Some(chosen) = dir_sizes.iter().find(|&&s| s >= needed_space) {
            println!("chose {chosen}");
        }
    }

    fn list_dirs(dir: Rc<RefCell<Dir>>, sizes: Rc<RefCell<Vec<usize>>>) {
        let this_dir = dir.borrow();
        sizes.borrow_mut().push(this_dir.total_size);
        for d in this_dir.subdirs.iter() {
            Self::list_dirs(Rc::clone(&d), Rc::clone(&sizes));
        }
    }

    fn du(&mut self, dir: Rc<RefCell<Dir>>) -> usize {
        let mut total;
        {
            let dir = (*dir).borrow();
            let files_size: usize = dir.files.iter().map(|f| (**f).borrow().size).sum();
            let dirs_size: usize = dir.subdirs.iter().map(|subdir|
                self.du(Rc::clone(subdir))
            ).sum();
            total = files_size + dirs_size;
            // println!("Total size for {} = {}", dir.name, total);
            if total < 100_000 {
                self.total += total;
            }
        }
        let mut foo = (*dir).borrow_mut();
        foo.total_size = total;
        return total;
    }


    fn parse_input(input: &String) -> Rc<RefCell<Dir>> {
        let root = Rc::new(RefCell::new(Dir::new(None, "/".to_string())));
        let mut current_dir = Rc::clone(&root);

        for line in input.lines() {
            if line.starts_with("$ cd ") {
                // change into a directory
                let dir = &line[5..];
                if dir == "/" {
                    current_dir = Rc::clone(&root);
                }
                else if dir == ".." {
                    let mut next_dir = Rc::clone(&root);
                    {
                        let parent = &current_dir.borrow().parent;
                        if parent.is_some() {
                            next_dir = Rc::clone(parent.as_ref().unwrap());
                        }
                    }
                    current_dir = next_dir;
                }
                else {
                    let that_dir = current_dir.borrow().subdirs.iter().find(|&d|
                        d.borrow().name == dir
                    ).expect("no dir found!").clone();
                    current_dir = that_dir;
                }
            }
            else if line.starts_with("$ ls") {
                // list files, useless to us
                // println!("listing at {}!", current_dir.borrow().name);
            }
            else if line.starts_with("dir ") {
                // create a new directory
                let dir = &line[4..];
                // println!("found dir: {}", dir);
                let created_dir = Rc::new(RefCell::new(
                    Dir::new(Some(Rc::clone(&current_dir)), dir.to_string())
                ));
                current_dir.borrow_mut().subdirs.push(created_dir);
            }
            else {
                // create a new file
                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                if parts.len() != 2 {
                    panic!("Fuck, didn't find 2 items");
                }
                if let [filesize, filename] = parts[0..2] {
                    let new_file = File {
                        parent: Rc::clone(&current_dir),
                        name: filename.to_string(),
                        size: filesize.parse().unwrap()
                    };
                    current_dir.borrow_mut().files.push(Rc::new(RefCell::new(new_file)));
                }

            }
        }

        return root;
    }
}
