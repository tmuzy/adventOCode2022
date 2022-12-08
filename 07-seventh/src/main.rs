use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

#[derive(Debug)]
struct ElfFile {
    name: String,
    size: i32,
    parent: Option<Arc<ElfFile>>,
}

impl ElfFile {
    fn is_dir(&self) -> bool {
        self.size < 0
    }
}

fn mkdir(name: &str) -> ElfFile {
    ElfFile {
        name: name.to_string(),
        size: -1,
        parent: None,
    }
}

fn main() {
    let history = get_lines("input");

    let tree_root = mkdir("/");
    let mut file_tree: Vec<Arc<ElfFile>> = vec![Arc::new(tree_root)];

    let mut current_directory = file_tree[0].clone();

    for item in &history[1..] {
        match get_command_type(item) {
            HistoryItemType::CD => {
                if let Some(result) = execute_cd(item, current_directory.clone()) {
                    current_directory = Arc::new(result);
                    file_tree.push(current_directory.clone());
                } else {
                    current_directory = current_directory.parent.clone().unwrap()
                }
            }
            HistoryItemType::FILE => {
                if !item.starts_with("dir") {
                    let mut split_item = item.split_whitespace();

                    let new_file = ElfFile {
                        size: split_item.next().unwrap().parse::<i32>().unwrap(),
                        name: split_item.next().unwrap().to_string(),
                        parent: Some(current_directory.clone()),
                    };
                    file_tree.push(Arc::new(new_file))
                }
            }
            HistoryItemType::LS => (),
        }
    }
    let mut dir_sizes = HashMap::new();
    for file in &file_tree[1..] {
        if !file.is_dir() {
            let parent = file.parent.clone().unwrap();
            let name = parent.name.clone();
            if dir_sizes.contains_key(&name) {
                let key = name.clone();
                dir_sizes.entry(key).and_modify(|size| *size += file.size);
            } else {
                dir_sizes.insert(name, file.size);
            }
        }
    }

    for (k, v) in dir_sizes.iter().filter(|dir| dir.1 <= &&100000) {
        println!("{}: {}", k, v);
    }
}

fn directory_size(dir: Arc<ElfFile>, tree: &Vec<Arc<ElfFile>>) {}

enum HistoryItemType {
    CD,
    LS,
    FILE,
}

fn execute_cd<'a>(command: &'a str, current_dir: Arc<ElfFile>) -> Option<ElfFile> {
    let dir_name = command.split_whitespace().last().unwrap();
    if dir_name != ".." {
        let new_file = ElfFile {
            name: dir_name.to_string(),
            size: -1,
            parent: Some(current_dir),
        };
        return Some(new_file);
    } else {
        return None;
    }
}

fn get_command_type(item: &str) -> HistoryItemType {
    if item.starts_with("$ cd") {
        return HistoryItemType::CD;
    }
    if item.starts_with("$ ls") {
        return HistoryItemType::LS;
    }
    HistoryItemType::FILE
}

fn get_lines(filename: &str) -> Vec<String> {
    BufReader::new(File::open(filename).expect("file not found"))
        .lines()
        .map(|l| l.expect("error line read"))
        .collect()
}
