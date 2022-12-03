use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

// Define a global hash map of constants
// A for Rock, B for Paper, and C for Scissors
// X for Rock, Y for Paper, and Z for Scissors
lazy_static! {
    static ref MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("A", "R");
        map.insert("B", "P");
        map.insert("C", "S");
        map.insert("X", "R");
        map.insert("Y", "P");
        map.insert("Z", "S");
        map
    };
}


pub fn solve() {
    if let Ok(lines) = read_file("data/day2.txt") {
        println!("Lines read in");
        let parts: Vec<&str> = lines.split("\n").collect();
        part1(parts);
    } else {
        println!("Failed");
    }

    if let Ok(lines) = read_file("data/day2.txt") {
        println!("Lines read in");
        let parts: Vec<&str> = lines.split("\n").collect();
        part2(parts);
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

fn part1(parts: Vec<&str>) {
    let mut total_points: i64 = 0;
    for part in parts {
        if part.is_empty() {
            continue;
        }
        let throws: Vec<&str> = part.split(" ").collect();
        let opp: &str = throws[0];
        let me: &str = throws[1];
        let points: i64 = calc_score((&opp, &me));
        total_points += points;
    }
    println!("Points: {}", total_points);
}

// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
fn part2(parts: Vec<&str>) {
    let mut total_points: i64 = 0;
    for part in parts {
        if part.is_empty() {
            continue;
        }
        let throws: Vec<&str> = part.split(" ").collect();
        let opp: &str = throws[0];
        let result: &str = throws[1];
        let convert: (&str, &str) = (MAP.get(opp).unwrap(), result);

        let points = match convert {
            ("R", "X") => 3 + 0, //3S + 0L
            ("R", "Y") => 1 + 3, //1R + 3D
            ("R", "Z") => 2 + 6, //2P + 6W
            ("P", "X") => 1 + 0, //1R + 0L
            ("P", "Y") => 2 + 3, //2P + 3D
            ("P", "Z") => 3 + 6, //3S + 6W
            ("S", "X") => 2 + 0, //2P + 0L
            ("S", "Y") => 3 + 3, //3S + 3D
            ("S", "Z") => 1 + 6, //1R + 6W
            (_, _) => -1,
        };
        total_points += points;
    }
    println!("Points: {}", total_points);
}

// 1 for Rock, 2 for Paper, and 3 for Scissors
// 0 if you lost, 3 if the round was a draw, and 6 if you won
fn calc_score(opp: (&str, &str)) -> i64 {
    let convert: (&str, &str) = (MAP.get(opp.0).unwrap(), MAP.get(opp.1).unwrap());
    match convert {
        ("R", "R") => 1 + 3, //1R + 3D
        ("R", "P") => 2 + 6, //2P + 6W
        ("R", "S") => 3 + 0, //3S + 0L
        ("P", "R") => 1 + 0, //1R + 0L
        ("P", "P") => 2 + 3, //2P + 2D
        ("P", "S") => 3 + 6, //3S + 6W
        ("S", "R") => 1 + 6, //1R + 6W
        ("S", "P") => 2 + 0, //2P + 0L
        ("S", "S") => 3 + 3, //3S + 3D
        (_, _) => -1,
    }
}