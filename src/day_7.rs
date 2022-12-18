use std::collections::HashMap;

struct Directory {
    /// List of full paths of subdirectories of this one
    subdirs: Vec<String>,
    
    /// List of (filename, size) tuples
    files: Vec<(String, u64)>,
}

impl Directory {
    fn new_empty() -> Self {
        Self {
            subdirs: Vec::new(),
            files: Vec::new(),
        }
    }
}

pub struct Filesystem {
    /// Maps full paths to Directories
    directories: HashMap<String, Directory>,
}

impl AsRef<Filesystem> for Filesystem {
    fn as_ref(&self) -> &Filesystem {
        self
    }
}

impl Filesystem {
    fn get_dir(&self, path: &str) -> Option<&Directory> {
        self.directories.get(path)
    }
    
    fn get_or_insert_dir_mut(&mut self, path: &str) -> &mut Directory {
        self.directories.entry(path.into())
            .or_insert(Directory::new_empty())
    }
}

pub fn parse(input: &str) -> Memo {
    let mut fs = Filesystem { directories: HashMap::new() };
    
    // Insert the root node first
    fs.directories.insert(String::from(""), Directory::new_empty());

    let mut current_path = Vec::<String>::new();
    let mut lines = input.lines().peekable();
    loop {
        let line = match lines.next() {
            Some(x) => x,
            None => break
        };

        if line.starts_with("$ cd") {
            let (_, new_path_elem) = line.split_at(5);
            match new_path_elem {
                ".." => { current_path.pop(); },
                "/" => current_path.clear(),
                other => current_path.push(other.into()),
            }
        } else {
            assert!(line == "$ ls");
            let dir = fs.get_or_insert_dir_mut(&current_path.join("/"));
            
            while lines.peek().map_or(false, |l| !l.starts_with("$")) {
                let mut parts = lines.next().unwrap().split_whitespace();
                let a = parts.next().unwrap();
                let b = parts.next().unwrap();
                
                if a == "dir" {
                    current_path.push(b.into());
                    let path = current_path.join("/");
                    dir.subdirs.push(path);
                    current_path.pop();
                } else {
                    let size = a.parse().expect("Failed to parse filesize");
                    dir.files.push((b.into(), size));
                }
            }
        }
    }

    form_memo(&fs)
}

pub struct Memo(HashMap<String, u64>);

fn form_memo(fs: &Filesystem) -> Memo {
    fn recurse(memo: &mut HashMap<String, u64>, fs: &Filesystem, path: &str, dir: &Directory) {
        for x in &dir.subdirs {
            let dir = match fs.get_dir(&x) {
                Some(x) => x,
                None => continue,
            };
            recurse(memo, fs, &*x, dir);
        }
        
        let files_size = dir
            .files
            .iter()
            .map(|(_name, size)| size)
            .sum::<u64>();
    
        let dir_sizes = dir
            .subdirs
            .iter()
            .map(|path| memo.get(path).unwrap_or(&0))
            .sum::<u64>();

        let size = files_size + dir_sizes;
        memo.insert(path.into(), size);
    }
    
    // Map path to directory size
    let mut memo = HashMap::new();
    recurse(&mut memo, fs, "", fs.get_dir("").unwrap());
    
    Memo(memo)
}

impl AsRef<Memo> for Memo {
    fn as_ref(&self) -> &Memo {
        self
    }
}

pub fn solve_part_1(memo: &Memo) -> u64 {
    memo.0
        .values()
        .filter(|x| **x <= 100_000)
        .sum()
}

pub fn solve_part_2(memo: &Memo) -> u64 {
    let disk_size = 70_000_000;
    let required_free = 30_000_000;
    let total_used = *memo.0.get("").unwrap();
    
    let required_delete = required_free - (disk_size - total_used);
    
    *memo.0
        .values()
        .filter(|x| **x >= required_delete)
        .min()
        .unwrap()
}