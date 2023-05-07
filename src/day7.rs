use std::collections::HashMap;

use crate::solver::Solver;

pub fn get_solver() -> Solver {
    Solver::new("./inputs/7.txt", |input| {
        let mut out = vec![];
        let mut file_system = FileSystem::new();

        let mut lines = input.lines();
        lines.next(); //skip root
        for line in lines {
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

        //Print fs for fun
        file_system.contents.values().for_each(|f| {
            let line = format!("{: <56} : {: <8}", f.path(), f.size);
            println!("{}", line);
        });
        println!();

        //Part 1
        let p1 = file_system
            .contents
            .values()
            .filter_map(|f| {
                if f.size <= 100_000 {
                    Some(f.size)
                } else {
                    None
                }
            })
            .reduce(|acc, x| acc + x)
            .unwrap()
            .to_string();
        out.push(p1);

        //Part 2
        let total_space = &file_system.contents.get("").unwrap().size;
        let empty_space = 70000000 - total_space;
        let space_needed = 30000000 - empty_space;
        let default_folder = Folder {
            size: u32::MAX,
            path: String::new(),
        };

        let p2 = file_system
            .contents
            .values()
            .fold(default_folder, |acc, f| {
                if f.size > space_needed && f.size < acc.size {
                    f.to_owned()
                } else {
                    acc
                }
            });
        out.push(p2.size.to_string());

        println!(
            "Need {} : Delete {} to save {}",
            space_needed,
            p2.path(),
            p2.size
        );
        out
    })
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
            contents,
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
        if let Some(split) = name.rsplit_once('/') {
            let parent = split.0.to_string();
            self.add_file_r(&parent, size);
        }
    }
    fn step_up(name: &str) -> String {
        name.rsplit_once('/').unwrap().0.to_string()
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

#[cfg(test)]
pub mod test {
    use crate::solver::Validate;

    use super::*;
    const TEST_PATH: &str = "./test_inputs/7.txt";
    const EXPECTED_TEST: (&str, &str) = ("95437", "24933642");
    const EXPECTED_REAL: (&str, &str) = ("1582412", "3696336");

    #[test]
    fn test() {
        let solver = get_solver();
        if let Err(e) = solver.validate_on(TEST_PATH, EXPECTED_TEST) {
            panic!("{:?}", e);
        }
    }

    #[test]
    fn real() {
        let solver = get_solver();
        if let Err(e) = solver.validate(EXPECTED_REAL) {
            panic!("{:?}", e);
        }
    }
}
