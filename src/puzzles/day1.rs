use std::fs::File;
use std::io::Read;

pub fn solve() {
    if let Ok(lines) = read_file("data/input.txt") {
        println!("Lines read in");
        let parts: Vec<&str> = lines.split("\n").collect();
        let mut calories: Vec<u32> = parse_lines(parts);
        
        calories.sort();
        calories.reverse();
        
        let first = calories[0];
        let second = calories[1];
        let third = calories[2];
        println!("MAX CARRYING: {}", first);
        println!("TOP 3 CARRYING: {}", (first + second + third))
    } else {
        println!("Failed");
    }
}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_lines(parts: Vec<&str>) -> Vec<u32> {
    let mut calories: Vec<u32> = vec![];
    let mut total_count = 0;

    for part in parts {
        if let Ok(count) = part.parse::<u32>() {
            total_count += count;
        } else {
            calories.push(total_count);
            total_count = 0;
        }
    }

    return calories;
}