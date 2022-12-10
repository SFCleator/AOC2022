use std::fs;

struct File {
    size: i32,
    name: String,
}

struct Directory {
    id: i32,
    children: Vec<i32>,
    files: Vec<File>,
    name: String,
}

impl Directory {
    pub fn get_size(&self, listing: &Vec<Directory>) -> i32 {
        let mut size = 0;

        for file in &self.files {
            size += file.size;
        }

        for other_dir in listing {
            if self.children.contains(&other_dir.id) {
                size += other_dir.get_size(listing);
            }
        }

        return size;
    }
}

struct FileSystem {
    root_id: i32,
    dirs: Vec<Directory>,
    max_id: i32,
}

impl FileSystem {
    pub fn new() -> Self {
        let mut fs = FileSystem {
            root_id: 0,
            dirs: Vec::new(),
            max_id: 0,
        };
        fs.dirs.push(Directory {
            id: 0,
            children: Vec::new(),
            files: Vec::new(),
            name: "/".to_string(),
        });

        return fs;
    }
    pub fn find_dir_id(&self, dir_name: &str, parent_id: i32) -> i32 {
        for dir in &self.dirs {
            if dir.id == parent_id {
                for child_id in &dir.children {
                    for child_dir in &self.dirs {
                        if child_dir.id == *child_id {
                            if child_dir.name == dir_name.to_string() {
                                return *child_id;
                            }
                        }
                    }
                }
            }
        }

        return 0;
    }

    pub fn find_parent_id(&self, child_id: i32) -> i32 {
        for dir in &self.dirs {
            if dir.children.contains(&child_id) {
                return dir.id;
            }
        }
        return 0;
    }

    pub fn add_directory(&mut self, parent_id: i32, name: &str) {
        self.max_id += 1;
        let new_dir = Directory {
            id: self.max_id,
            children: Vec::new(),
            files: Vec::new(),
            name: name.to_string(),
        };
        self.dirs.push(new_dir);

        for dir in &mut self.dirs {
            if dir.id == parent_id {
                dir.children.push(self.max_id);
            }
        }
    }

    pub fn add_file(&mut self, directory_id: i32, name: &str, size: i32) {
        for dir in &mut self.dirs {
            if dir.id == directory_id {
                dir.files.push(File {
                    size: size,
                    name: name.to_string(),
                });
            }
        }
    }

    pub fn print(&self) {
        for dir in &self.dirs {
            println!("{}", dir.name);
            for file in &dir.files {
                println!("-- {} : {}", file.name, file.size);
            }
        }
    }

    pub fn get_directory_sizes(&self) -> Vec<i32> {
        let mut sizes: Vec<i32> = Vec::new();

        for dir in &self.dirs {
            sizes.push(dir.get_size(&self.dirs));
        }

        return sizes;
    }
}

struct Terminal {
    pwd: i32,
    filesystem: FileSystem,
}

impl Terminal {
    pub fn new() -> Self {
        let fs = FileSystem::new();

        Terminal {
            pwd: fs.root_id,
            filesystem: fs,
        }
    }

    pub fn parse(&mut self, command_and_output: &str) {
        let mut output: Vec<&str> = command_and_output.split("\n").collect();
        let mut args: Vec<&str> = output.remove(0).split(" ").collect();
        let command_name = args.remove(0);

        match command_name {
            "ls" => self.ls(output),
            "cd" => self.cd(args),
            &_ => println!("Command \"{}\" not recognised", command_name),
        }
    }

    pub fn ls(&mut self, output: Vec<&str>) {
        for line in output {
            if line == "" {
                continue;
            }
            let elements: Vec<&str> = line.split(" ").collect();
            match *elements.get(0).unwrap() {
                "dir" => self
                    .filesystem
                    .add_directory(self.pwd, elements.get(1).unwrap()),
                x => self.filesystem.add_file(
                    self.pwd,
                    elements.get(1).unwrap(),
                    x.to_string().parse().unwrap(),
                ),
            };
        }
    }

    pub fn cd(&mut self, args: Vec<&str>) {
        if args.get(0).unwrap().to_string() == ".." {
            self.pwd = self.filesystem.find_parent_id(self.pwd);
        } else {
            self.pwd = self.filesystem.find_dir_id(args.get(0).unwrap(), self.pwd);
        }
    }
}

fn main() {
    let file_path = String::from("input.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let commands: Vec<&str> = contents.split("$ ").collect();

    let mut terminal = Terminal::new();

    for command in commands {
        if command == "" {
            continue;
        }

        terminal.parse(command);
    }

    let mut small_sizes = 0;
    const CUTOFF_SIZE: i32 = 100000;
    let dir_sizes = terminal.filesystem.get_directory_sizes();
    for size in &dir_sizes {
        if *size <= CUTOFF_SIZE {
            small_sizes += *size;
        }
    }
    println!("Cutoff Sizes: {small_sizes}");

    const TOTAL_DISK_SPACE: i32 = 70000000;
    const DISK_SPACE_NEEDED: i32 = 30000000;
    let mut max_size = 0;
    for size in &dir_sizes {
        if *size > max_size {
            max_size = *size;
        }
    }
    let free_up_required = DISK_SPACE_NEEDED - TOTAL_DISK_SPACE + max_size;
    let mut smallest_free_up = max_size;
    for size in &dir_sizes {
        if *size > free_up_required && *size < smallest_free_up {
            smallest_free_up = *size;
        }
    }
    println!("Smallest free up needed: {smallest_free_up}");
}
