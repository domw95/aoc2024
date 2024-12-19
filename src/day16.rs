use std::collections::HashMap;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use aoc_tools::grid::Coord;
use aoc_tools::grid::Grid;

type Input = String;

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn cost(&self, other: &Self) -> usize {
        match (self, other) {
            (Orientation::North, Orientation::North) => 0,
            (Orientation::North, Orientation::East) => 1000,
            (Orientation::North, Orientation::South) => 2000,
            (Orientation::North, Orientation::West) => 1000,
            (Orientation::East, Orientation::North) => 1000,
            (Orientation::East, Orientation::East) => 0,
            (Orientation::East, Orientation::South) => 1000,
            (Orientation::East, Orientation::West) => 2000,
            (Orientation::South, Orientation::North) => 2000,
            (Orientation::South, Orientation::East) => 1000,
            (Orientation::South, Orientation::South) => 0,
            (Orientation::South, Orientation::West) => 1000,
            (Orientation::West, Orientation::North) => 1000,
            (Orientation::West, Orientation::East) => 2000,
            (Orientation::West, Orientation::South) => 1000,
            (Orientation::West, Orientation::West) => 0,
        }
    }
}

fn find_path(
    grid: &Grid<u8>,
    pos: Coord,
    orientation: Orientation,
    cache: &mut HashMap<(Coord, Orientation), Option<usize>>,
    visited: &mut Grid<bool>,
) -> Option<usize> {
    // dbg!(&pos);
    if let Some(&cell) = grid.checked_index(&pos) {
        if cell == b'#' {
            None
        } else if cell == b'E' {
            Some(0)
        } else if visited[pos] {
            None
        } else {
            //

            if let Some(Some(cached)) = cache.get(&(pos, orientation)) {
                Some(*cached)
            } else {
                visited[pos] = true;
                let scores = [
                    find_path(grid, pos.north(), Orientation::North, cache, visited)
                        .map(|score| score + orientation.cost(&Orientation::North) + 1),
                    find_path(grid, pos.east(), Orientation::East, cache, visited)
                        .map(|score| score + orientation.cost(&Orientation::East) + 1),
                    find_path(grid, pos.south(), Orientation::South, cache, visited)
                        .map(|score| score + orientation.cost(&Orientation::South) + 1),
                    find_path(grid, pos.west(), Orientation::West, cache, visited)
                        .map(|score| score + orientation.cost(&Orientation::West) + 1),
                ];
                let best = scores.into_iter().flatten().min();

                cache.insert((pos, orientation), best);

                visited[pos] = false;
                best
            }
        }
    } else {
        None
    }
}

#[aoc(day16, part1)]
fn solver_part1(input: &Input) -> usize {
    let width = input.lines().next().unwrap().len();
    let grid = Grid::from_iter(&mut input.lines().flat_map(|l| l.bytes()), width);
    // dbg!(&grid);
    // let mut cache = Grid::from_iter(&mut (0..(grid.height * grid.width)).map(|_| None), width);
    let mut cache = HashMap::new();
    let mut visited = Grid::from_iter(&mut (0..(grid.height * grid.width)).map(|_| false), width);
    let mut pos = Coord::new(0, 0);
    let direction = Orientation::East;
    for (c, i) in grid.iter() {
        if *i == b'S' {
            pos = c;
            break;
        }
    }
    // dbg!(&pos);

    find_path(&grid, pos, direction, &mut cache, &mut visited).unwrap()
}

#[aoc(day16, part2)]
fn solver_part2(_input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    static INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    static INPUT3: &str = "###########################
#######################..E#
######################..#.#
#####################..##.#
####################..###.#
###################..##...#
##################..###.###
#################..####...#
################..#######.#
###############..##.......#
##############..###.#######
#############..####.......#
############..###########.#
###########..##...........#
##########..###.###########
#########..####...........#
########..###############.#
#######..##...............#
######..###.###############
#####..####...............#
####..###################.#
###..##...................#
##..###.###################
#..####...................#
#.#######################.#
#S........................#
###########################";
    static INPUT4: &str = "##########
#.......E#
#.##.#####
#..#.....#
##.#####.#
#S.......#
##########";

    static INPUT5: &str = "########################################################
#.........#.........#.........#.........#.........#...E#
#.........#.........#.........#.........#.........#....#
#....#....#....#....#....#....#....#....#....#....#....#
#....#....#....#....#....#....#....#....#....#....#....#
#....#....#....#....#....#....#....#....#....#....#....#
#....#....#....#....#....#....#....#....#....#....#....#
#....#.........#.........#.........#.........#.........#
#S...#.........#.........#.........#.........#.........#
########################################################";

    static INPUT6: &str = "..E
...
...
S..";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 7036)
    }

    #[test]
    fn part1_2() {
        assert_eq!(solver_part1(&input_generator(INPUT2)), 11048)
    }

    #[test]
    fn part1_3() {
        assert_eq!(solver_part1(&input_generator(INPUT3)), 21148)
    }

    #[test]
    fn part1_4() {
        assert_eq!(solver_part1(&input_generator(INPUT4)), 4013)
    }

    #[test]
    fn part1_5() {
        assert_eq!(solver_part1(&input_generator(INPUT5)), 21110)
    }

    #[test]
    fn part1_6() {
        assert_eq!(solver_part1(&input_generator(INPUT6)), 1005)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 0)
    }
}
