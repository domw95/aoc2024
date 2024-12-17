use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Input = String;

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn parse_line(l: &str) -> ((i32, i32), (i32, i32)) {
    let (px, l) = l.split_once('=').unwrap().1.split_once(',').unwrap();
    let (py, l) = l.split_once(' ').unwrap();
    let (vx, vy) = l.split_once('=').unwrap().1.split_once(',').unwrap();
    let pos: (i32, i32) = (px.parse().unwrap(), py.parse().unwrap());
    let vel: (i32, i32) = (vx.parse().unwrap(), vy.parse().unwrap());
    (pos, vel)
}
fn step(pos: &mut (i32, i32), vel: &(i32, i32)) {
    pos.0 += vel.0;
    pos.1 += vel.1;
    if pos.0 < 0 {
        pos.0 += 101
    } else if pos.0 >= 101 {
        pos.0 -= 101
    }
    if pos.1 < 0 {
        pos.1 += 103
    } else if pos.1 >= 103 {
        pos.1 -= 103
    }
}

#[aoc(day14, part1)]
fn solver_part1(input: &Input) -> u32 {
    input
        .lines()
        .fold([0, 0, 0, 0], |mut q, l| {
            //
            let (mut pos, vel) = parse_line(l);
            for _ in 0..100 {
                step(&mut pos, &vel);
            }
            #[allow(clippy::comparison_chain)]
            if pos.0 < 50 {
                if pos.1 < 51 {
                    q[0] += 1
                } else if pos.1 > 51 {
                    q[1] += 1
                }
            } else if pos.0 > 50 {
                if pos.1 < 51 {
                    q[2] += 1
                } else if pos.1 > 51 {
                    q[3] += 1
                }
            }
            q
        })
        .iter()
        .product()
}

#[aoc(day14, part1, REM)]
fn solver_part1_rem(input: &Input) -> u32 {
    input
        .lines()
        .fold([0, 0, 0, 0], |mut q, l| {
            let (mut pos, vel) = parse_line(l);
            pos.0 = (pos.0 + 100 * vel.0).rem_euclid(101);
            pos.1 = (pos.1 + 100 * vel.1).rem_euclid(103);
            #[allow(clippy::comparison_chain)]
            if pos.0 < 50 {
                if pos.1 < 51 {
                    q[0] += 1
                } else if pos.1 > 51 {
                    q[1] += 1
                }
            } else if pos.0 > 50 {
                if pos.1 < 51 {
                    q[2] += 1
                } else if pos.1 > 51 {
                    q[3] += 1
                }
            }
            q
        })
        .iter()
        .product()
}

#[aoc(day14, part2)]
fn solver_part2(input: &Input) -> u32 {
    let mut bots = input.lines().map(parse_line).collect_vec();
    let mut steps = 0;
    loop {
        steps += 1;
        let mut grid = [[false; 101]; 103];
        for bot in &mut bots {
            step(&mut bot.0, &bot.1);
            grid[bot.0 .1 as usize][bot.0 .0 as usize] = true;
        }
        let row_count = grid
            .iter()
            .map(|row| {
                let (max, current) = row.iter().fold((0, 0), |(max, current), item| {
                    if *item {
                        (max, current + 1)
                    } else if current > max {
                        (current, 0)
                    } else {
                        (max, 0)
                    }
                });
                max.max(current)
            })
            .filter(|&count| count > 10)
            .count();
        if row_count > 5 {
            // check for tree
            println!("===============================================================");
            for row in grid {
                for cell in row {
                    if cell {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            println!();
            return steps;
        }
    }
}

#[aoc(day14, part2, BORDER)]
fn solver_part2_border(input: &Input) -> u32 {
    let mut bots = input.lines().map(parse_line).collect_vec();
    let mut steps = 0;
    loop {
        steps += 1;
        let mut grid = [[false; 101]; 103];
        for bot in &mut bots {
            step(&mut bot.0, &bot.1);
            grid[bot.0 .1 as usize][bot.0 .0 as usize] = true;
        }
        if grid.iter().any(|row| {
            let (max, current) = row.iter().fold((0, 0), |(max, current), item| {
                if *item {
                    (max, current + 1)
                } else if current > max {
                    (current, 0)
                } else {
                    (max, 0)
                }
            });
            max.max(current) == 31
        }) {
            return steps;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;

    static INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 12)
    }
}
