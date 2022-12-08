use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
enum Path {
    Root,
    Dir(String),
    Up,
}

//struct FullPath

impl Path {
    fn from_string(s: &str) -> Self {
        match s {
            "/" => Self::Root,
            ".." => Self::Up,
            _ => Self::Dir(String::from(s)),
        }
    }
}

#[derive(Debug)]
enum Result {
    Dir(String),
    File(String, u32),
}

impl Result {
    fn from_string(s: &str) -> Option<Result> {
        lazy_static! {
            static ref FILE_RE: Regex = Regex::new(r"^(?P<size>[\d]+) (?P<file>.*)$").unwrap();
            static ref DIR_RE: Regex = Regex::new(r"^dir (?P<dir>.*)$").unwrap();
        }

        match (FILE_RE.captures(s), DIR_RE.captures(s)) {
            (Some(_), Some(_)) => panic!("what?"),
            (Some(fmatch), _) => Some(Self::File(
                String::from(fmatch.name("file").unwrap().as_str()),
                fmatch.name("size").unwrap().as_str().parse().unwrap(),
            )),
            (None, Some(dmatch)) => Some(Self::Dir(String::from(
                dmatch.name("dir").unwrap().as_str(),
            ))),
            (None, None) => None,
        }
    }
}

#[derive(Debug)]
enum Command {
    Cd(Path),
    Ls,
}

impl Command {
    fn from_string(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RES: [(&'static str, Regex); 2] = [
                ("cd", Regex::new(r"^\$ cd (?P<dir>.+)$").unwrap()),
                ("ls", Regex::new(r"^\$ ls$").unwrap())
            ];
        }
        let m = RES
            .iter()
            .find_map(|(which, re)| re.captures(s).map(|captures| (*which, captures)));

        match m? {
            ("cd", captures) => Some(Self::Cd(Path::from_string(
                captures.name("dir").unwrap().as_str(),
            ))),
            ("ls", _captures) => Some(Self::Ls),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum CommandOrResult {
    Command(Command),
    Result(Result),
}

impl CommandOrResult {
    fn from_string(s: &str) -> Option<CommandOrResult> {
        let c = Command::from_string(s);
        let c = c.map(|c| Self::Command(c));
        c.or_else(|| Result::from_string(s).map(|o| Self::Result(o)))
    }
}

#[derive(Debug)]
struct Dir {
    name: String,
    subdirs: Vec<Rc<RefCell<Dir>>>,
    files: HashMap<String, u32>,
}

impl Dir {
    fn from_name(s: &str) -> Self {
        Self {
            name: String::from(s),
            subdirs: Vec::new(),
            files: HashMap::new(),
        }
    }

    fn size(&self) -> u32 {
        let my_size: u32 = self.files.values().sum();
        let children_sizes: u32 = self.subdirs.iter().map(|d| d.borrow().size()).sum();
        my_size + children_sizes
    }

    // I really should make an iterator but I'm lazy...
    fn collect(n: Rc<RefCell<Dir>>) -> Vec<Rc<RefCell<Dir>>> {
        let mut children: Vec<Rc<RefCell<Dir>>> = n
            .borrow()
            .subdirs
            .iter()
            .flat_map(|c| Self::collect(c.clone()))
            .collect();
        children.push(n);
        children
    }
}

fn main() {
    let ok = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| CommandOrResult::from_string(&l));
    let move_it = ok.map(|o| o.unwrap());
    let moves: Vec<_> = move_it.collect();
    //println!("cool {:?}", moves);

    let root = Rc::new(RefCell::new(Dir::from_name("/")));
    let mut dir_stack: Vec<Rc<RefCell<Dir>>> = Vec::from([root.clone()]);

    for m in moves.iter() {
        match m {
            CommandOrResult::Command(Command::Ls) => (),
            CommandOrResult::Command(Command::Cd(new_path)) => {
                match new_path {
                    Path::Root => dir_stack.truncate(1),

                    Path::Up => {
                        dir_stack.pop();
                    }
                    Path::Dir(d) => {
                        let current_dir = dir_stack.last().unwrap();
                        //let subdirs = &current_dir.subdirs;

                        //let current_dir_borrow = current_dir.borrow();

                        let existing_dir = current_dir
                            .borrow()
                            .subdirs
                            .iter()
                            .find(|sd| sd.borrow().name == *d)
                            .map(Rc::clone);
                        let dir = existing_dir.unwrap_or_else(|| {
                            let newdir = Rc::new(RefCell::new(Dir::from_name(d)));
                            // Add to subdirs...
                            current_dir.borrow_mut().subdirs.push(newdir.clone());
                            newdir
                        });

                        dir_stack.push(dir);
                    }
                }
            }
            CommandOrResult::Result(Result::Dir(_)) => (), // don't care
            CommandOrResult::Result(Result::File(file, size)) => {
                //println!("Dir: {:?} File: {file} Size: {size}", dir_stack);
                // copies file
                dir_stack
                    .last_mut()
                    .unwrap()
                    .borrow_mut()
                    .files
                    .insert(file.to_string(), *size);
            }
        }
    }

    let collection = Dir::collect(root.clone());
    let small_dirs = collection
        .iter()
        .map(|d| d.borrow().size())
        .filter(|sz| *sz <= 100000);
    println!("part 1 sum: {}", small_dirs.sum::<u32>());

    let total_disk = 70000000 as u32;
    let need = 30000000 as u32;
    let total_used = root.borrow().size();
    let total_free = total_disk - total_used;
    let need_to_delete = need - total_free;
    println!("total used {total_used} total_free {total_free}");
    println!("need to rm {need_to_delete}");

    // What is the smallest dir that will make total_size - sz <= 30000000?

    let min_dir = collection
        .iter()
        .filter(|d| d.borrow().size() >= need_to_delete)
        .min_by_key(|d| d.borrow().size())
        .unwrap();

    println!("min size = {}", min_dir.borrow().size());

}
