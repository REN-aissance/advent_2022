use std::collections::HashMap;

pub fn run(s: String) {
    let mut file_system = FileSystem::new();

    let mut lines = s.lines();
    lines.next(); //skip root
    while let Some(line) = lines.next() {
        let args = line.split_whitespace().collect::<Vec<&str>>();
        match args[0] {
            "$" => {
                if args[1] == "cd" {
                    file_system.cd(args[2]);
                }
            }
            "dir" => file_system.add_folder(args[1]),
            _ => file_system.add_file(args[0].parse::<u32>().unwrap()),
        }
    }

    //Part 1
    let mut total = 0;
    for f in file_system.contents.values() {
        if f.size <= 100000 {
            total += f.size;
        }
        let line = format!("{: <56} : {: <8} : {}", f.path(), f.size, total);
        println!("{}", line);
    }
    println!();

    //Part 2
    let total_space = &file_system.contents.get("").unwrap().size;
    let empty_space = 70000000 - total_space;
    let space_needed = 30000000 - empty_space;

    let mut candidate_directory = Folder::default();
    candidate_directory.add_file(u32::MAX);

    for f in file_system.contents.values() {
        if f.size > space_needed && f.size < candidate_directory.size {
            candidate_directory = f.clone();
        }
    }
    println!(
        "Need {} : Delete {} to save {}",
        space_needed,
        candidate_directory.path(),
        candidate_directory.size
    );
}

struct FileSystem {
    contents: HashMap<String, Folder>,
    working_directory: String,
}
impl FileSystem {
    fn new() -> FileSystem {
        let mut contents: HashMap<String, Folder> = HashMap::new();
        contents.insert("".to_string(), Folder::default());
        FileSystem {
            contents: contents,
            working_directory: "".to_string(),
        }
    }
    fn add_folder(&mut self, name: &str) {
        let folder = Folder::new(name.to_string(), self.working_directory.clone());
        self.contents.insert(folder.path.clone(), folder);
    }
    fn cd(&mut self, name: &str) {
        match name {
            ".." => self.working_directory = FileSystem::step_up(&self.working_directory),
            _ => {
                self.working_directory += "/";
                self.working_directory += name;
            }
        }
    }
    fn add_file(&mut self, size: u32) {
        self.add_file_r(&self.working_directory.clone(), size);
    }
    fn add_file_r(&mut self, name: &String, size: u32) {
        self.contents.get_mut(name).unwrap().add_file(size);
        if let Some(split) = name.rsplit_once("/") {
            let parent = split.0.to_string();
            self.add_file_r(&parent, size);
        }
    }
    fn step_up(name: &String) -> String {
        name.rsplit_once("/").unwrap().0.to_string()
    }
}

#[derive(Clone)]
struct Folder {
    size: u32,
    path: String,
}

impl Folder {
    fn default() -> Self {
        Folder {
            size: 0,
            path: "".to_string(),
        }
    }
    fn new(name: String, parent: String) -> Self {
        Folder {
            size: 0,
            path: parent + "/" + &name,
        }
    }
    fn add_file(&mut self, size: u32) {
        self.size += size;
    }
    fn path(&self) -> String {
        if self.path.is_empty() {
            "/".to_string()
        } else {
            self.path.clone()
        }
    }
}
