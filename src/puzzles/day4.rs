use crate::utils;

pub fn solve() {
    if let Ok(lines) = utils::util_funcs::read_file("data/day4.txt") {
        println!("Lines read in");
        let parts: Vec<&str> = lines.split("\n").collect();
        count_contains(&parts);
    } else {
        println!("File read failed");
    }
}

fn count_contains(lines: &Vec<&str>) {
    let mut count = 0;
    let mut count_overlap = 0;

    for line in lines {
        let (f_start, f_end, b_start, b_end) = parse_line(line);
        if check_contains(f_start, f_end, b_start, b_end) {
            count += 1;
        } else if check_overlap(f_start, f_end, b_start, b_end) {
            count_overlap += 1;
        }
    }

    println!("Part 1 Count: {}", count);
    println!("Part 2 Count: {}", (count + count_overlap));
}

fn parse_line(line: &str) -> (i32, i32, i32, i32) {
    let sets = line.split(",").collect::<Vec<&str>>();
    let f_vals = sets[0].split("-").collect::<Vec<&str>>();
    let b_vals = sets[1].split("-").collect::<Vec<&str>>();
    println!("f_start: {}, f_end: {}, b_start: {}, b_end: {}", f_vals[0], f_vals[1], b_vals[0], b_vals[1]);
    return (f_vals[0].parse::<i32>().unwrap(), f_vals[1].parse::<i32>().unwrap(),
                b_vals[0].parse::<i32>().unwrap(), b_vals[1].parse::<i32>().unwrap());
}

fn check_contains(f_start: i32, f_end: i32, b_start: i32, b_end: i32) -> bool {
    if (f_start <= b_start) && (b_end <= f_end) {
        return true;
    } else if (b_start <= f_start) && (f_end <= b_end) {
        return true;
    } else {
        return false;
    }
}

fn check_overlap(f_start: i32, f_end: i32, b_start: i32, b_end: i32) -> bool {
    if f_start <= b_end {
        if f_end < b_start {
            return false;
        }
        return true;
    } else {
        return false;
    }
}