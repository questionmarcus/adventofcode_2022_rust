use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(PartialEq, Debug)]
enum ParseResult {
    ChangeDirectoryDown(String),
    ChangeDirectoryUp,
    ListDirectory,
    File { name: String, size: u32 },
    DescendantDir(String),
}

impl From<&str> for ParseResult {
    fn from(input: &str) -> Self {
        let mut input_iter = input.split_whitespace();
        match input_iter.next().unwrap() {
            "$" => {
                if input_iter.next().unwrap() == "cd" {
                    let arg: &str = input_iter.next().unwrap();
                    if arg == ".." {
                        ParseResult::ChangeDirectoryUp
                    } else {
                        ParseResult::ChangeDirectoryDown(arg.to_owned())
                    }
                } else {
                    ParseResult::ListDirectory
                }
            }
            "dir" => ParseResult::DescendantDir(input_iter.next().unwrap().to_owned()),
            n => ParseResult::File {
                name: input_iter.next().unwrap().to_owned(),
                size: n.parse::<u32>().unwrap(),
            },
        }
    }
}

fn get_dir_sizes(input: &str) -> HashMap<PathBuf, u32> {
    let mut current_path: PathBuf = Path::new("/").to_path_buf();
    let mut dir_sizes: HashMap<PathBuf, u32> = HashMap::new();

    for line in input.lines() {
        match ParseResult::from(line) {
            ParseResult::ListDirectory => {
                dir_sizes.insert(current_path.clone(), 0);
            }
            ParseResult::ChangeDirectoryDown(directory) => {
                current_path = append_dir(current_path, directory)
            }
            ParseResult::ChangeDirectoryUp => current_path = strip_last_dir(current_path),
            ParseResult::File { size, .. } => {
                match dir_sizes.get_mut(&current_path) {
                    Some(volume) => *volume += size,
                    None => {
                        dir_sizes.insert(current_path.clone(), size);
                    }
                };

                let mut parent_path = current_path.clone();
                while parent_path.pop() {
                    if let Some(volume) = dir_sizes.get_mut(&parent_path) {
                        *volume += size
                    }
                }
            }
            ParseResult::DescendantDir(_) => (),
        }
    }
    dir_sizes
}

fn append_dir(path: PathBuf, dir: String) -> PathBuf {
    path.join(dir)
}

fn strip_last_dir(path: PathBuf) -> PathBuf {
    match path.parent() {
        Some(path) => path.to_path_buf(),
        None => path,
    }
}

pub fn part_one(input: &str) -> u32 {
    let dir_sizes = get_dir_sizes(input);

    dir_sizes
        .values()
        .filter(|total_size| *total_size <= &100000)
        .sum::<u32>()
}

pub fn part_two(input: &str) -> u32 {
    let dir_sizes = get_dir_sizes(input);

    let available_disk_space = 70000000 - *dir_sizes.get(&Path::new("/").to_path_buf()).unwrap();
    let space_required = 30000000 - available_disk_space;

    *dir_sizes
        .values()
        .filter(|total_size| *total_size >= &space_required)
        .min()
        .unwrap()
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
dir b
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 95437);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 24933642)
    }

    #[test]
    fn parsing_line() {
        assert_eq!(ParseResult::from("$ ls"), ParseResult::ListDirectory);
        assert_eq!(
            ParseResult::from("$ cd /"),
            ParseResult::ChangeDirectoryDown(String::from("/"))
        );
        assert_eq!(ParseResult::from("$ cd .."), ParseResult::ChangeDirectoryUp);
        assert_eq!(
            ParseResult::from("dir a"),
            ParseResult::DescendantDir(String::from("a"))
        );
        assert_eq!(
            ParseResult::from("12345 a.txt"),
            ParseResult::File {
                name: String::from("a.txt"),
                size: 12345
            }
        );
    }

    #[test]
    fn append_dir_to_path() {
        assert_eq!(
            append_dir(Path::new("/test/path").to_path_buf(), "added".to_owned()),
            Path::new("/test/path/added").to_path_buf()
        );
        assert_eq!(
            append_dir(Path::new("/").to_path_buf(), "path".to_owned()),
            Path::new("/path").to_path_buf()
        )
    }

    #[test]
    fn remove_last_dir() {
        assert_eq!(
            strip_last_dir(Path::new("/test/path").to_path_buf()),
            Path::new("/test")
        );
        assert_eq!(strip_last_dir(Path::new("/").to_path_buf()), Path::new("/"));
    }
}
