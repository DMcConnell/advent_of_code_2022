use std::fs::File;
use std::io::Read;
use std::env;
use std::path::PathBuf;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

fn hashset(data: &[u8]) -> HashSet<u8> {
    HashSet::from_iter(data.iter().cloned())
}

pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let cwd = get_current_working_dir();
    println!{"Working in directory: {}", cwd.unwrap().display()};
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

pub fn vec_to_set(vec: &Vec<char>) -> HashSet<char> {
    HashSet::from_iter(vec.iter().cloned())
}

pub fn intersection_of_sets(vec1: &Vec<char>, vec2: &Vec<char>) -> Vec<char> {
    vec1.iter().filter(|x| vec2.contains(x)).cloned().collect()
}