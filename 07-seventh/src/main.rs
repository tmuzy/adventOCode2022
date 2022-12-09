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

    fn is_root(&self) -> bool {
        self.name == "/"
    }
}

fn main() {
    let history = get_lines("input");

    let tree_root = Arc::new(ElfFile {
        name: "/".to_string(),
        size: -1,
        parent: None,
    });
    let mut file_tree: Vec<Arc<ElfFile>> = vec![tree_root.clone()];

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

    let mut dir_under_100000_sum = 0;
    file_tree.sort_by(|a, b| get_file_depth(a.clone()).cmp(&get_file_depth(b.clone())));
    for file in &file_tree {
        if file.is_dir() {
            let dir_size = directory_size(file.clone(), &file_tree);
            if dir_size <= 100000 {
                dir_under_100000_sum += dir_size
            }
            println!(
                "{} f {} - {}",
                "-".repeat(get_file_depth(file.clone())),
                file.name,
                dir_size
            );
        } else {
            println!(
                "{} d {} - {}",
                "-".repeat(get_file_depth(file.clone())),
                file.name,
                file.size
            );
        }
    }
    println!("{}", dir_under_100000_sum);
}

fn directory_size(dir: Arc<ElfFile>, tree: &Vec<Arc<ElfFile>>) -> i32 {
    let mut sum = 0;
    for file in tree {
        if !file.is_root() {
            if dir.name == file.parent.clone().unwrap().name {
                if !file.is_dir() {
                    sum += file.size;
                }
            }
        }
    }
    sum
}

fn get_file_depth(file: Arc<ElfFile>) -> usize {
    if file.is_root() {
        return 0;
    }
    let mut depth: usize = 1;
    let mut search = file.clone();
    while search.parent.clone().unwrap().name != "/" {
        depth += 1;
        search = search.parent.clone().unwrap();
    }
    depth
}

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
