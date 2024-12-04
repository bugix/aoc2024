use std::fs::read_to_string;
use crate::day2::Direction::{Decreasing, Increasing};

#[derive(Debug)]
enum Direction {
    Increasing,
    Decreasing
}

pub(crate) fn day2() {
    let lines = read_lines("day2.txt");

    let mut total1 = 0;

    for line in lines {
        let numbers = line.split(" ").collect::<Vec<&str>>();

        let mut direction: Option<Direction> = None;
        let mut previous_number = 0;
        let mut safe = true;
        let mut problem_dampener = true;

        for number in numbers {
            let number = number.parse::<i32>().unwrap();

            if previous_number == 0 {
                previous_number = number;
            }
            else {
                if direction.is_none() {
                    if number > previous_number {
                        direction = Some(Increasing);
                    }

                    if number < previous_number {
                        direction = Some(Decreasing);
                    }
                }

                match direction {
                    Some(Increasing) => {
                        let increase = number - previous_number;
                        if increase < 1 || increase > 3 {
                            if problem_dampener {
                                problem_dampener = false;
                            }
                            else {
                                safe = false;
                                break;
                            }
                        }
                    }
                    Some(Decreasing) => {
                        let decrease = previous_number - number;
                        if decrease < 1 || decrease > 3 {
                            if problem_dampener {
                                problem_dampener = false;
                            }
                            else {
                                safe = false;
                                break;
                            }
                        }
                    }
                    None => {
                        if problem_dampener {
                            problem_dampener = false;
                        }
                        else {
                            safe = false;
                            break;
                        }
                    }
                }

                previous_number = number;
            }
        }

        if safe {
            total1 += 1;
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
