use std::{
    cell::Cell,
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

#[derive(Debug)]
struct ElfFile {
    name: String,
    size: i64,
    parent: Option<Arc<ElfFile>>,
    total_size: Cell<i64>,
    depth: usize,
}

impl ElfFile {
    fn is_dir(&self) -> bool {
        self.size < 0
    }

    fn is_root(&self) -> bool {
        self.name == "/"
    }

    fn get_content(&self, tree: &Vec<Arc<ElfFile>>) -> Option<Vec<Arc<ElfFile>>> {
        if !self.is_dir() {
            return None;
        }
        let mut files: Vec<Arc<ElfFile>> = Vec::new();
        for file in tree {
            if !file.is_root() {
                if self.name == file.parent.clone().unwrap().name {
                    files.push(file.clone())
                }
            }
        }
        if files.len() > 0 {
            return Some(files);
        }
        None
    }
}

fn main() {
    let history = get_lines("input");

    let file_tree = build_file_tree(&history);

    print_tree(&file_tree);

    let sum = file_tree
        .iter()
        .filter(|f| f.total_size.get() <= 100000 && f.total_size.get() > 0)
        .map(|f| f.total_size.get())
        .sum::<i64>();

    println!("sum of dirs < 100000 : {}", sum);

    let space_used = file_tree
        .iter()
        .find(|f| f.is_root())
        .unwrap()
        .total_size
        .get();

    const SPACE_TOTAL: i64 = 70_000_000;
    const SPACE_NEEDED: i64 = 30_000_000;
    println!("free space {}", SPACE_TOTAL - space_used);
    let space_to_free = SPACE_NEEDED - (SPACE_TOTAL - space_used);
    println!("space to free : {}", space_to_free);

    let to_remove_size = file_tree
        .iter()
        .filter(|f| f.total_size.get() > space_to_free)
        .map(|f| f.total_size.get())
        .min()
        .unwrap();

    println!("dir to delete {}", to_remove_size);
}

fn print_tree(file_tree: &Vec<Arc<ElfFile>>) {
    for file in file_tree.iter().filter(|f| f.is_dir()) {
        if let Some(content) = file.get_content(file_tree) {
            print_content(file, &content);
        }
    }
}

fn print_content(file: &Arc<ElfFile>, content: &Vec<Arc<ElfFile>>) {
    println!(
        "{} ┍{} ({})",
        " ".repeat(file.depth) + "",
        file.name,
        file.total_size.get(),
    );
    let mut subfolders = content.iter().filter(|f| !f.is_dir()).peekable();

    while let Some(content_file) = subfolders.next() {
        if let Some(sub) = content_file.get_content(content) {
            print_content(content_file, &sub);
        }
        println!(
            "{}┄┄{} ({})",
            if subfolders.peek().is_none() {
                " ".repeat(file.depth) + " └"
            } else {
                " ".repeat(file.depth) + " ├"
            },
            content_file.name,
            content_file.size
        );
    }
}

fn build_file_tree(history: &Vec<String>) -> Vec<Arc<ElfFile>> {
    let tree_root = Arc::new(ElfFile {
        name: "/".to_string(),
        size: -1,
        parent: None,
        total_size: Cell::new(0),
        depth: 0,
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
                        size: split_item.next().unwrap().parse::<i64>().unwrap(),
                        name: split_item.next().unwrap().to_string(),
                        parent: Some(current_directory.clone()),
                        total_size: Cell::new(0),
                        depth: current_directory.depth + 1,
                    };

                    let file_arc = Arc::new(new_file);
                    file_tree.push(file_arc.clone());
                    let mut parent = file_arc.parent.clone().unwrap();
                    while !parent.is_root() {
                        parent
                            .total_size
                            .set(parent.total_size.get() + file_arc.clone().size);
                        parent = parent.clone().parent.clone().unwrap()
                    }
                    if parent.is_root() {
                        parent
                            .total_size
                            .set(parent.total_size.get() + file_arc.clone().size)
                    }
                }
            }
            HistoryItemType::LS => (),
        }
    }

    file_tree
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
            parent: Some(current_dir.clone()),
            total_size: Cell::new(0),
            depth: current_dir.clone().depth + 1,
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
