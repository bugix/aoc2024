use std::fs::read_to_string;

pub(crate) fn day4() {
    let lines = read_lines("day4.txt");

    let mut total1 = 0;
    let mut grid = [[0; 140]; 140];
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                'X' => grid[y][x] = 1,
                'M' => grid[y][x] = 2,
                'A' => grid[y][x] = 3,
                'S' => grid[y][x] = 4,
                _ => continue,
            }
        }
    }

    for y in 1..139 {
        for x in 1..139 {

            if grid[y][x] == 3 {
                if ((grid[y - 1][x - 1] == 2 && grid[y + 1][x + 1] == 4) || (grid[y - 1][x - 1] == 4 && grid[y + 1][x + 1] == 2))
                    &&
                    ((grid[y + 1][x - 1] == 2 && grid[y - 1][x + 1] == 4) || (grid[y + 1][x - 1] == 4 && grid[y - 1][x + 1] == 2))
                    {
                        total1 += 1;
                    }
            }

            /*

            // EAST
            if x < 137 {
                if grid[y][x] == 1 &&
                    grid[y][x + 1] == 2 &&
                    grid[y][x + 2] == 3 &&
                    grid[y][x + 3] == 4 {
                    total1 += 1;
                }
            }

            // SOUTH-EAST
            if x < 137 && y < 137 {
                if grid[y][x] == 1 &&
                    grid[y + 1][x + 1] == 2 &&
                    grid[y + 2][x + 2] == 3 &&
                    grid[y + 3][x + 3] == 4 {
                    total1 += 1;
                }
            }

            // SOUTH
            if y < 137 {
                if grid[y][x] == 1 &&
                    grid[y + 1][x] == 2 &&
                    grid[y + 2][x] == 3 &&
                    grid[y + 3][x] == 4 {
                    total1 += 1;
                }
            }

            // SOUTH-WEST
            if x > 2 && y < 137 {
                if grid[y][x] == 1 &&
                    grid[y + 1][x - 1] == 2 &&
                    grid[y + 2][x - 2] == 3 &&
                    grid[y + 3][x - 3] == 4 {
                    total1 += 1;
                }
            }

            // WEST
            if x > 2 {
                if grid[y][x] == 1 &&
                    grid[y][x - 1] == 2 &&
                    grid[y][x - 2] == 3 &&
                    grid[y][x - 3] == 4 {
                    total1 += 1;
                }
            }

            // NORTH-WEST
            if x > 2 && y > 2 {
                if grid[y][x] == 1 &&
                    grid[y - 1][x - 1] == 2 &&
                    grid[y - 2][x - 2] == 3 &&
                    grid[y - 3][x - 3] == 4 {
                    total1 += 1;
                }
            }

            // NORTH
            if y > 2 {
                if grid[y][x] == 1 &&
                    grid[y - 1][x] == 2 &&
                    grid[y - 2][x] == 3 &&
                    grid[y - 3][x] == 4 {
                    total1 += 1;
                }
            }

            // NORTH-EAST
            if x < 137 && y > 2 {
                if grid[y][x] == 1 &&
                    grid[y - 1][x + 1] == 2 &&
                    grid[y - 2][x + 2] == 3 &&
                    grid[y - 3][x + 3] == 4 {
                    total1 += 1;
                }
            }
             */

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
