use std::fs::read_to_string;

pub(crate) fn day1() {
    let lines = read_lines("day1.txt");
    let mut from_points = Vec::new();
    let mut to_points = Vec::new();
    for line in lines {
        let parts = line.split("   ").collect::<Vec<&str>>();
        from_points.push(parts[0].parse::<i32>().unwrap());
        to_points.push(parts[1].parse::<i32>().unwrap());
    }

    from_points.sort();
    to_points.sort();

    let mut total1 = 0;
    let mut total2 = 0;

    for i in 0..from_points.len() {
        let distance = i32::abs(from_points[i] - to_points[i]);
        total1 += distance;

        let count = to_points.iter().filter(|x| **x == from_points[i]).count() as i32 * from_points[i];
        total2 += count;
    }

    println!("part 1: {} part 2: {}", total1, total2);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        lines.push(line.to_string());
    }

    lines
}
