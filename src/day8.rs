use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Input = String;

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

#[aoc(day8, part1)]
fn solver_part1(input: &Input) -> usize {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut total = 0;
    let mut length = 0;
    for (i, line) in input.lines().enumerate() {
        length += 1;

        for (j, c) in line.chars().enumerate() {
            total += 1;
            if c != '.' {
                match map.get_mut(&c) {
                    Some(vec) => {
                        vec.push((i as i32, j as i32));
                    }
                    None => {
                        map.insert(c, vec![(i as i32, j as i32)]);
                    }
                }
            }
        }
    }
    let width = total / length;
    let mut set = HashSet::new();
    for vec in map.values() {
        for (a, b) in vec.iter().tuple_combinations() {
            let diff = (b.0 - a.0, b.1 - a.1);

            let pos = (b.0 + diff.0, b.1 + diff.1);
            if !(pos.0 < 0 || pos.1 < 0 || pos.0 >= width || pos.1 >= length) {
                set.insert(pos);
            }
            let pos = (a.0 - diff.0, a.1 - diff.1);
            if !(pos.0 < 0 || pos.1 < 0 || pos.0 >= width || pos.1 >= length) {
                set.insert(pos);
            }
        }
    }

    // dbg!(map);
    set.len()
}

#[aoc(day8, part2)]
fn solver_part2(input: &Input) -> usize {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut total = 0;
    let mut length = 0;
    for (i, line) in input.lines().enumerate() {
        length += 1;

        for (j, c) in line.chars().enumerate() {
            total += 1;
            if c != '.' {
                match map.get_mut(&c) {
                    Some(vec) => {
                        vec.push((i as i32, j as i32));
                    }
                    None => {
                        map.insert(c, vec![(i as i32, j as i32)]);
                    }
                }
            }
        }
    }
    let width = total / length;

    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for vec in map.values() {
        for (a, b) in vec.iter().tuple_combinations() {
            set.insert(*a);
            set.insert(*b);
            let diff = (b.0 - a.0, b.1 - a.1);

            let mut i = 1;
            loop {
                let pos = (b.0 + diff.0 * i, b.1 + diff.1 * i);
                if !(pos.0 < 0 || pos.1 < 0 || pos.0 >= width || pos.1 >= length) {
                    set.insert(pos);
                } else {
                    break;
                }
                i += 1;
            }

            let mut i = 1;
            loop {
                let pos = (a.0 - diff.0 * i, a.1 - diff.1 * i);
                if !(pos.0 < 0 || pos.1 < 0 || pos.0 >= width || pos.1 >= length) {
                    set.insert(pos);
                } else {
                    break;
                }
                i += 1;
            }
        }
    }

    // dbg!(map);
    set.len()
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 14)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 34)
    }
}
