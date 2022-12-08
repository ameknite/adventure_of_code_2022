use std::{
    env,
    error::{self, Error},
    fs::{self, File},
    io,
    path::Path,
    str::FromStr,
};

pub fn input() -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string("input.txt")?)
}

// Advent of Code 2022
// --- Day 7: No Space Left On Device ---

struct MyFile {
    name: String,
    size: u32,
}

impl FromStr for MyFile {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, name) = s.split_once(' ').ok_or("Error parsing file")?;
        Ok(MyFile {
            name: name.to_string(),
            size: size.parse()?,
        })
    }
}

struct MyDir {
    name: String,
    size: Option<u32>,
}

impl FromStr for MyDir {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((_, dir)) = s.split_once("$ cd ") {
            return Ok(MyDir {
                name: dir.to_string(),
                size: None,
            });
        }
        let (_, dir) = s.split_once("dir ").ok_or("Error parsing Dir")?;
        Ok(MyDir {
            name: dir.to_string(),
            size: None,
        })
    }
}

struct FileSystem {
    root: MyDir,
    input: String,
    disk_space: u32,
}

impl FromStr for FileSystem {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (root, input) = s.split_once('\n').ok_or("Error parsing input")?;
        let (_, mut root) = root.split_once("$ cd ").ok_or("Error parsing root")?;
        if root == "/" {
            root = "/root"
        }
        Ok(Self {
            root: MyDir {
                name: env::current_dir()?
                    .to_str()
                    .ok_or("Error parsing root")?
                    .to_string()
                    + root,
                size: None,
            },
            input: input.to_string(),
            disk_space: 70_000_000,
        })
    }
}

impl FileSystem {
    fn create_file_system(&self) -> Result<(), Box<dyn Error>> {
        // Clean file system
        if self.root_as_path().exists() {
            std::fs::remove_dir_all(self.root_as_path())?;
        }
        // Root
        self.mkdir(&self.root)?;
        self.cd(&self.root)?;
        // FileSystem
        for line in self.input.lines() {
            if line.starts_with("$ cd ") {
                let dir = line.parse::<MyDir>()?;
                self.cd(&dir)?;
            } else if line.starts_with("dir") {
                let dir = line.parse::<MyDir>()?;
                self.mkdir(&dir)?;
            } else if line
                .chars()
                .next()
                .ok_or("Error parsing line")?
                .is_numeric()
            {
                let file = line.parse::<MyFile>()?;
                self.touch(&file)?;
            }
        }

        self.return_to_origin()?;
        Ok(())
    }

    fn cd(&self, dir: &MyDir) -> io::Result<()> {
        std::env::set_current_dir(&dir.name)?;
        Ok(())
    }

    fn mkdir(&self, dir: &MyDir) -> io::Result<()> {
        fs::create_dir_all(&dir.name)?;
        Ok(())
    }

    fn touch(&self, file: &MyFile) -> io::Result<()> {
        File::create(format!("{}_{}", file.size, file.name))?;
        Ok(())
    }

    fn return_to_origin(&self) -> Result<(), Box<dyn Error>> {
        std::env::set_current_dir(format!("{}{}", &self.root.name, "/.."))?;
        Ok(())
    }

    fn root_as_path(&self) -> &Path {
        Path::new(&self.root.name)
    }

    fn get_file_with_size(&self, path: &Path) -> Result<MyFile, Box<dyn Error>> {
        let data = path
            .file_name()
            .ok_or("Error reading file name")?
            .to_str()
            .ok_or("Error reading file name")?;
        let (size, name) = data.split_once('_').ok_or("Error reading file name")?;
        Ok(MyFile {
            name: name.to_string(),
            size: size.parse()?,
        })
    }

    fn get_dir_with_size(&self, path: &Path) -> Result<MyDir, Box<dyn Error>> {
        let size = self.calculate_size_directory(path)?;
        let name = path
            .file_name()
            .ok_or("Error reading dir name")?
            .to_str()
            .ok_or("Error reading dir name")?;
        Ok(MyDir {
            name: name.to_string(),
            size: Some(size),
        })
    }

    fn calculate_size_directory(&self, path: &Path) -> Result<u32, Box<dyn Error>> {
        let mut total = 0;
        for path in fs::read_dir(path)
            .expect("Error finding root directory")
            .into_iter()
            .flatten()
            .map(|entry| entry.path())
        {
            if path.is_file() {
                total += self.get_file_with_size(&path)?.size;
            }
            if path.is_dir() {
                total += self.calculate_size_directory(&path)?;
            }
        }

        Ok(total)
    }

    fn directories_with_at_most_size(
        &self,
        path: &Path,
        size: u32,
    ) -> Result<Vec<MyDir>, Box<dyn Error>> {
        let mut directories = Vec::<MyDir>::new();
        let dir = self.get_dir_with_size(path)?;
        if dir.size <= Some(size) {
            directories.push(dir);
        }
        for path in fs::read_dir(path)
            .expect("Error finding directory")
            .into_iter()
            .flatten()
            .map(|entry| entry.path())
        {
            if path.is_dir() {
                directories.append(&mut self.directories_with_at_most_size(&path, size)?);
            }
        }
        Ok(directories)
    }

    fn directories_with_at_least_size(
        &self,
        path: &Path,
        size: u32,
    ) -> Result<Vec<MyDir>, Box<dyn Error>> {
        let mut directories = Vec::<MyDir>::new();
        let dir = self.get_dir_with_size(path)?;

        if dir.size >= Some(size) {
            directories.push(dir);
        }
        for path in fs::read_dir(path)
            .expect("Error finding directory")
            .into_iter()
            .flatten()
            .map(|entry| entry.path())
        {
            if path.is_dir() {
                directories.append(&mut self.directories_with_at_least_size(&path, size)?);
            }
        }
        Ok(directories)
    }
}

pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let file_system = input.parse::<FileSystem>()?;
    file_system.create_file_system()?;

    let at_most_size = 100000;
    let directories =
        file_system.directories_with_at_most_size(file_system.root_as_path(), at_most_size)?;

    let sum_directories_sizes = directories
        .iter()
        .flat_map(|directory| directory.size)
        .sum();
    Ok(sum_directories_sizes)
}

pub fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    let file_system = input.parse::<FileSystem>()?;
    file_system.create_file_system()?;

    let size_for_the_update = 30_000_000;
    let used_storage = file_system.calculate_size_directory(file_system.root_as_path())?;
    let unused_storage = file_system.disk_space - used_storage;
    let file_size_to_delete = size_for_the_update - unused_storage;

    let directories = file_system
        .directories_with_at_least_size(file_system.root_as_path(), file_size_to_delete)?;

    let size_min_directory = directories
        .iter()
        .flat_map(|directory| directory.size)
        .min()
        .ok_or("Error finding file to delete")?;
    Ok(size_min_directory)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, 24933642);
    }
}
