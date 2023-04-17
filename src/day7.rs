use std::{iter::Peekable, str::Lines};

pub fn run(s: String) {
    let mut lines = s.lines().peekable();

    let root = Folder::default();
    let mut parent = root;
    while lines.peek().is_some() {
        let args = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();
        match args[0] {
            "$" => match args[1] {
                "cd" => parent = parent,
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

trait Item {
    fn size(&self) -> u32;
    fn from(name: &str, size: &str) -> Self
    where
        Self: Sized;
}
struct File {
    name: &'static str,
    size: u32,
}
struct Folder {
    parent: Option<&'static Folder>,
    contents: Vec<Box<dyn Item>>,
    name: &'static str,
    size: u32,
}

impl Folder {
    fn add(&mut self, file: File) {
        self.size += file.size;
        self.contents.push(Box::new(file));
    }
    fn default() -> Self {
        Folder {
            parent: None,
            contents: Vec::new(),
            name: "/",
            size: 0,
        }
    }
}

impl Item for File {
    fn size(&self) -> u32 {
        self.size
    }

    fn from(name: &'static str, size: &str) -> Self
    where
        Self: Sized,
    {
        File {
            name: name,
            size: size.parse().unwrap(),
        }
    }
}

impl Item for Folder {
    fn size(&self) -> u32 {
        self.size
    }
    fn from(name: &str, size: &str) -> Self
    where
        Self: Sized,
    {
        Folder {
            parent: todo!(),
            name: name,
            size: size.parse().unwrap(),
            contents: Vec::new(),
        }
    }
}
