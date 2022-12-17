use crate::utils;

pub fn solve() {
    if let Ok(lines) = utils::util_funcs::read_file("data/day3.txt") {
        println!("Lines read in");
        let parts: Vec<&str> = lines.split("\n").collect();
        let value: i32 = sum_parts(&parts);
        println!("Part 1 Sum: {}", value);

        let badge_value: i32 = find_badges(&parts);
        println!{"Part 2 Sum: {}", badge_value};
    } else {
        println!("File read failed");
    }
}

fn sum_parts(lines: &Vec<&str>) -> i32 {
    let mut sum: i32 = 0;

    for line in lines {
        let (front, back) = line.split_at(line.len()/2);
        let front_chars: Vec<char> = front.chars().collect();
        let back_chars: Vec<char> = back.chars().collect();
        let intersection: Vec<char> = front_chars.iter().filter(|x| back_chars.contains(x)).cloned().collect();
        let value: i32 = calc_value(intersection[0]);
        sum += value;
        println!("Intersection: {:?}, value: {}", intersection, value);
    }
    sum
}

fn find_badges(lines: &Vec<&str>) -> i32 {
    let mut sum: i32 = 0;
    let mut grouper: Vec<&str> = vec![];

    for line in lines {
        println!("Line: {:?}", line);
        if grouper.len() < 2 {
            grouper.push(line);
            continue;
        }
        let front_chars: Vec<char> = grouper[0].chars().collect();
        let mid_chars: Vec<char> = grouper[1].chars().collect();
        let back_chars: Vec<char> = line.chars().collect();

        let intersection: Vec<char> = front_chars.iter().filter(|x| mid_chars.contains(x)).cloned().collect();
        let fin_int: Vec<char> = intersection.iter().filter(|x| back_chars.contains(x)).cloned().collect();

        let mut value: i32 = 0;
        for letter in fin_int.iter() {
            value = calc_value(*letter);
        }
        sum += value;
        println!("{:?}\t{:?}\t{:?}", front_chars, mid_chars, back_chars);
        println!("Intersection: {:?}, value: {}", fin_int, value);

        grouper = vec![];
    }
    sum
}

fn calc_value(letter: char) -> i32 {
    let dist_from_low: i32 = letter as i32 - 'a' as i32;
    if dist_from_low > 0 {
        return dist_from_low + 1;
    } else {
        let dist_from_cap: i32 = letter as i32 - 'A' as i32;
        return (dist_from_cap + 1) + 26;
    }
}