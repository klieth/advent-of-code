use std::collections::HashMap;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;

const TEST_DATA: &'static str = include_str!("../../test");
const INPUT_DATA: &'static str = include_str!("../../input");

const FS_SIZE: usize = 70000000;
const REQUIRED_SIZE: usize = 30000000;

#[derive(Debug)]
enum Day7Error {
    ParseError(String),
    StateError(String),
}

impl From<ParseIntError> for Day7Error {
    fn from(error: ParseIntError) -> Self {
        Day7Error::ParseError(format!("failed to parse int: {}", error))
    }
}

#[derive(Debug)]
enum Entry {
    File(String, usize),
    Dir(String),
}

impl FromStr for Entry {
    type Err = Day7Error;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let (fst, name) = raw.split_once(" ")
            .ok_or_else(|| Day7Error::ParseError(format!("failed to split ls output line: {}", raw)))?;

        if fst == "dir" {
            Ok(Entry::Dir(name.to_string()))
        } else {
            let size = fst.parse()?;
            Ok(Entry::File(name.to_string(), size))
        }
    }
}

// this assumes that dirs and files have distinct namespaces.
#[derive(Debug)]
struct Directory {
    dirs: HashMap<String, Directory>,
    files: HashMap<String, usize>
}

impl Directory {
    fn new() -> Self {
        Directory {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn size(&self) -> usize {
        let mut res = 0;

        for (_, size) in self.files.iter() {
            res += size;
        }

        for (_, dir) in self.dirs.iter() {
            res += dir.size();
        }

        res
    }

    fn dirs_lt(&self, lt: usize) -> usize {
        let mut res = 0;

        let size = self.size();
        if size < lt {
            res += size;
        }

        for (_, dir) in self.dirs.iter() {
            res += dir.dirs_lt(lt);
        }

        res
    }

    fn find_smallest_gt(&self, gt: usize) -> Option<&Self> {
        let size = self.size();
        if size <= gt {
            return None;
        }

        let min = self.dirs.iter().filter_map(|(_, d)| d.find_smallest_gt(gt))
            .min_by_key(|d| d.size());

        match min {
            Some(m) => Some(if m.size() < size { m } else { self }),
            None => Some(self),
        }
    }
}

struct TraversalState {
    cwd_path: Vec<String>,
    root: Directory,
}

impl TraversalState {
    fn new() -> Self {
        Self {
            cwd_path: vec![],
            root: Directory::new(),
        }
    }

    fn get_cwd_mut(&mut self) -> Option<&mut Directory> {
        let mut cwd = Some(&mut self.root);
        for segment in self.cwd_path.iter() {
            cwd = cwd.unwrap().dirs.get_mut(segment);
            if cwd.is_none() {
                return None;
            }
        }
        return cwd;
    }

    fn cd(&mut self, dir: &str) -> Result<(), Day7Error> {
        if dir == ".." {
            return if let Some(_) = self.cwd_path.pop() {
                Ok( () )
            } else {
                Err(Day7Error::StateError(format!("Cannot traverse above the root directory")))
            };
        }

        let cwd = self.get_cwd_mut()
            .ok_or_else(|| Day7Error::StateError(format!("Failed to get current working directory")))?;

        if cwd.dirs.contains_key(dir) {
            self.cwd_path.push(dir.to_string());
        } else {
            // input data is assumed to always view directories before attempting to traverse to
            // them.
            unreachable!("cd: directory does not exist")
        }

        Ok( () )
    }

    fn mkdir(&mut self, name: String) -> Result<(), Day7Error> {
        let cwd = self.get_cwd_mut()
            .ok_or_else(|| Day7Error::StateError(format!("Failed to get current working directory")))?;

        cwd.dirs.insert(name, Directory::new());

        Ok( () )
    }

    fn touch(&mut self, name: String, size: usize) -> Result<(), Day7Error> {
        let cwd = self.get_cwd_mut()
            .ok_or_else(|| Day7Error::StateError(format!("Failed to get current working directory")))?;

        cwd.files.insert(name, size);

        Ok( () )
    }

    fn consume(self) -> Directory {
        self.root
    }
}

trait Command: Debug {
    fn run(&self, state: &mut TraversalState) -> Result<(), Day7Error>;
    fn add_output(&mut self, output: &str) -> Result<(), Day7Error>;
}

#[derive(Debug)]
struct Cd {
    dir: String,
}

impl Command for Cd {
    fn run(&self, state: &mut TraversalState) -> Result<(), Day7Error> {
        state.cd(&self.dir)?;

        Ok( () )
    }

    fn add_output(&mut self, _output: &str) -> Result<(), Day7Error> {
        unreachable!("cd never has any output to be parsed")
    }
}

#[derive(Debug)]
struct Ls {
    output: Vec<Entry>,
}

impl Ls {
    fn new() -> Self {
        Ls {
            output: vec![],
        }
    }
}

impl Command for Ls {
    fn run(&self, state: &mut TraversalState) -> Result<(), Day7Error> {
        for entry in self.output.iter() {
            match entry {
                Entry::Dir(n) => state.mkdir(n.to_string())?,
                Entry::File(n, s) => state.touch(n.to_string(), *s)?,
            };
        }

        Ok( () )
    }

    fn add_output(&mut self, output: &str) -> Result<(), Day7Error> {
        self.output.push(output.parse()?);
        Ok( () )
    }
}

fn parse(raw: &str) -> Result<Directory, Day7Error> {
    let mut commands: Vec<Box<dyn Command>> = vec![];

    for line in raw.lines() {
        if let Some(raw_command) = line.strip_prefix("$ ") {
            if let Some(dir) = raw_command.strip_prefix("cd ") {
                commands.push(Box::new(Cd { dir: dir.to_string() }));
            } else if let Some(_) = raw_command.strip_prefix("ls") {
                commands.push(Box::new(Ls::new()));
            } else {
                unreachable!("input is verified to only contain cd and ls commands");
            }
        } else {
            let last: &mut Box<dyn Command> = commands.last_mut().expect("got file output before any commands were run");
            last.add_output(line)?;
        }
    }

    let mut state = TraversalState::new();

    // skip the first command, since we assume it always changes to the root directory
    for command in commands.iter().skip(1) {
        command.run(&mut state)?;
    }

    Ok(state.consume())
}

fn part1(root_dir: &Directory) -> usize {
    root_dir.dirs_lt(100000)
}

fn part2(root_dir: &Directory) -> usize {
    let unused = FS_SIZE - root_dir.size();
    let still_need = REQUIRED_SIZE - unused;

    match root_dir.find_smallest_gt(still_need) {
        Some(d) => d.size(),
        None => if root_dir.size() > still_need { root_dir.size() } else { unreachable!("no directories were the proper size") },
    }
}

fn main() -> Result<(), Day7Error> {
    let test_data = parse(TEST_DATA)?;
    let input_data = parse(INPUT_DATA)?;
    println!("part1 (test): {:?}", part1(&test_data));
    println!("part1 (actual): {:?}", part1(&input_data));
    println!("part2 (test): {:?}", part2(&test_data));
    println!("part2 (actual): {:?}", part2(&input_data));
    Ok( () )
}
