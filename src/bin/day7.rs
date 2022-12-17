use anyhow::Result;
use std::{cell::RefCell, collections::HashMap, fs::read_to_string, rc::Rc};

const TOTAL_SIZE: usize = 70000000;
const REQUIRE_SIZE: usize = 30000000;

enum Entry {
    Dir(String),
    File(usize, String),
}

impl TryFrom<&Vec<&str>> for Entry {
    type Error = anyhow::Error;

    fn try_from(value: &Vec<&str>) -> Result<Self, Self::Error> {
        if value[0] == "dir" {
            Ok(Entry::Dir(value[1].to_string()))
        } else {
            Ok(Entry::File(value[0].parse()?, value[1].to_string()))
        }
    }
}

#[derive(Debug, Clone)]
struct Dir {
    pub files: HashMap<String, Rc<RefCell<Dir>>>,
    pub is_file: bool,
    pub size: usize,
}

impl Default for Dir {
    fn default() -> Self {
        Self {
            files: HashMap::new(),
            is_file: false,
            size: 0,
        }
    }
}

impl Dir {
    pub fn new(size: usize) -> Self {
        Self {
            files: HashMap::new(),
            is_file: true,
            size,
        }
    }
    pub fn add(&mut self, entry: Entry) {
        match entry {
            Entry::Dir(name) => {
                self.files
                    .insert(name, Rc::new(RefCell::new(Dir::default())));
            }
            Entry::File(size, name) => {
                self.size += size;
                self.files
                    .insert(name, Rc::new(RefCell::new(Dir::new(size))));
            }
        }
    }
}

fn dfs_p1(root: Rc<RefCell<Dir>>) -> usize {
    let mut size = if root.borrow().size <= 100000 {
        root.borrow().size
    } else {
        0
    };

    for (_, dir) in root.borrow().files.iter() {
        if !dir.borrow().is_file {
            size += dfs_p1(dir.clone());
        }
    }
    size
}

fn dfs_p2(root: Rc<RefCell<Dir>>, min_size: &mut usize, min_del_size: usize) {
    *min_size = (*min_size).min(root.borrow().size);

    for (_, dir) in root.borrow().files.iter() {
        if dir.borrow().size >= min_del_size {
            dfs_p2(dir.clone(), min_size, min_del_size);
        }
    }
}

fn main() -> Result<()> {
    let input = read_to_string("data/day7.txt")?;

    let root = Rc::new(RefCell::new(Dir::default()));
    let mut stack = Vec::new();
    let mut curr = root.clone();
    for line in input.lines().skip(1) {
        let arr = line.split_whitespace().collect::<Vec<_>>();

        if let Ok(e) = (&arr).try_into() {
            curr.borrow_mut().add(e);
        } else if arr[1] == "cd" {
            if arr[2] == ".." {
                let prev: Rc<RefCell<Dir>> = stack.pop().unwrap();
                prev.borrow_mut().size += curr.borrow().size;
                curr = prev;
            } else {
                let temp = curr.borrow().files[arr[2]].clone();
                stack.push(curr.clone());
                curr = temp;
            }
        }
    }

    while let Some(prev) = stack.pop() {
        prev.borrow_mut().size += curr.borrow().size;
        curr = prev;
    }

    println!("part1: {}", dfs_p1(root.clone()));
    let min_del_size = REQUIRE_SIZE - (TOTAL_SIZE - root.borrow().size);

    let mut min_size = usize::MAX;
    dfs_p2(root, &mut min_size, min_del_size);

    println!("part2: {}", min_size);
    Ok(())
}
