use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;
use std::collections::HashMap;

enum Command<'a> {
    Ls,
    Cd(&'a str),
    Dir(&'a str),
    File(&'a str, usize),
}

fn main() {
    let reader = BufReader::new(File::open("input/day07.txt").unwrap());

    let mut fs = HashMap::<String, usize>::new();
    fs.insert("/".into(), 0);
    let mut cwd = String::new();
    for line in reader.lines().map(Result::unwrap) {
        match parse(&line) {
            Command::Ls => (),
            Command::Cd(path) => match path {
                "/" => cwd = path.to_string(),
                ".." => cwd = parent(&cwd).to_string(),
                name => { cwd += name; cwd += "/"; }
            }
            Command::Dir(name) => add_dir(&mut fs, &cwd, name),
            Command::File(name, size) => add_file(&mut fs, &cwd, name, size),
        }
    }

    let total_size: usize = fs.iter().filter_map(|(path, &size)| {
        if path.ends_with('/') && size <= 100_000 {
            Some(size)
        } else {
            None
        }
    }).sum();

    println!("The sum of the total size of the small directories is {total_size}");

    let used_space = fs.get("/").unwrap();
    let required_space = 30_000_000 - (70_000_000 - used_space);
    let dir_size: usize = fs.iter().filter_map(|(path, &size)| {
        if path.ends_with('/') && size >= required_space {
            Some(size)
        } else {
            None
        }
    }).min().unwrap();

    println!("The size of the smallest suitable directory is {dir_size}");
}

fn parse(line: &str) -> Command {
    if line == "$ ls" {
        Command::Ls
    } else if let Some(("", path)) = line.split_once("$ cd ") {
        Command::Cd(path)
    } else if let Some(("", name)) = line.split_once("dir ") {
        Command::Dir(name)
    } else if let Some((a, b)) = line.split_once(' ') {
        Command::File(b, usize::from_str(a).unwrap())
    } else {
        panic!("Unrecognized line {line:?}")
    }
}

fn parent(path: &str) -> &str {
    let mut i = path.rfind('/').unwrap();
    if i+1 == path.len() {
        i = path[..i].rfind('/').unwrap();
    }
    &path[..=i]
}

fn add_dir(fs: &mut HashMap::<String, usize>, cwd: &str, name: &str) {
    assert!(cwd.ends_with('/'));
    let path: String = [cwd, name, "/"].into_iter().collect();
    assert!(!fs.contains_key(&path[..path.len()-1]));
    let check = fs.insert(path, 0);
    assert!(check.is_none());
}

fn add_file(fs: &mut HashMap::<String, usize>, cwd: &str, name: &str, size: usize) {
    assert!(cwd.ends_with('/'));
    let path: String = [cwd, name].into_iter().collect();
    let check = fs.insert(path.clone(), size);
    assert!(check.is_none());
    let mut p = parent(&path);
    loop {
        let entry = fs.get_mut(p).unwrap();
        *entry += size;
        if p == "/" { break }
        p = parent(p);
    }
}
