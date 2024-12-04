use std::fs::read_to_string;
use regex::Regex;

pub(crate) fn day3() {
    let lines = read_lines("day3.txt");

    let mut total1 = 0;
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut on = true;

    for line in lines {
        for capture in re.captures_iter(line.as_str()) {
            let op = capture.get(0).unwrap().as_str();

            if op == "don't()" {
                on = false;
            }
            else if op == "do()" {
                on = true;
            }
            else {
                if on {
                    let i1 = capture[1].parse::<i32>().unwrap();
                    let i2 = capture[2].parse::<i32>().unwrap();

                    total1 += i1 * i2;
                }
            }
        }
    }

    println!("{:?}", total1);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        lines.push(line.to_string());
    }

    lines
}
