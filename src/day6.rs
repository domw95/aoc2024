use std::collections::HashSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use aoc_tools::grid::Coord;
use aoc_tools::grid::Grid;
use rayon::prelude::*;

type Input = Grid<Square>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Square {
    Empty,
    Visited,
    Obstruction,
    Guard(Direction),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[aoc_generator(day6)]
fn input_generator(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    Grid::from_iter(
        &mut input.lines().flat_map(|l| {
            l.chars().map(|c| match c {
                '.' => Square::Empty,
                '#' => Square::Obstruction,
                '^' => Square::Guard(Direction::Up),
                '>' => Square::Guard(Direction::Right),
                '<' => Square::Guard(Direction::Left),
                'v' => Square::Guard(Direction::Down),
                _ => panic!(),
            })
        }),
        width,
    )
}

fn run_grid(grid: &mut Grid<Square>, mut current: Coord, mut direction: Direction) {
    loop {
        let (iter, next) = match direction {
            Direction::Up => (grid.north_iter_mut(current), Direction::Right),
            Direction::Right => (grid.east_iter_mut(current), Direction::Down),
            Direction::Down => (grid.south_iter_mut(current), Direction::Left),
            Direction::Left => (grid.west_iter_mut(current), Direction::Up),
        };
        current = iter
            .take_while(|(sq, _)| match **sq {
                Square::Empty => true,
                Square::Visited => true,
                Square::Obstruction => false,
                Square::Guard(_) => panic!(),
            })
            .map(|(sq, pos)| {
                *sq = Square::Visited;
                pos
            })
            .last()
            .unwrap();
        if current.x == 0
            || current.y == 0
            || current.x == (grid.width - 1) as i32
            || current.y == (grid.height - 1) as i32
        {
            break;
        } else {
            direction = next;
        }
    }
}

#[aoc(day6, part1)]
fn solver_part1(input: &Input) -> usize {
    let mut grid = input.clone();
    let mut current = Coord::new(0, 0);
    let mut direction = Direction::Up;
    for coord in grid.coord_iter() {
        match grid[coord] {
            Square::Guard(d) => {
                direction = d;
                current = coord;
                grid[coord] = Square::Visited;
                break;
            }
            _ => continue,
        }
    }

    run_grid(&mut grid, current, direction);

    grid.items
        .iter()
        .filter(|&sq| matches!(sq, Square::Visited))
        .count()
}

fn is_loop(grid: &mut Grid<Square>, mut current: Coord, mut direction: Direction) -> bool {
    let mut visited = HashSet::new();
    // visited.insert((current, direction));
    loop {
        let (iter, next) = match direction {
            Direction::Up => (grid.north_iter_mut(current), Direction::Right),
            Direction::Right => (grid.east_iter_mut(current), Direction::Down),
            Direction::Down => (grid.south_iter_mut(current), Direction::Left),
            Direction::Left => (grid.west_iter_mut(current), Direction::Up),
        };
        current = iter
            .take_while(|(sq, _)| match **sq {
                Square::Empty => true,
                Square::Visited => true,
                Square::Obstruction => false,
                Square::Guard(_) => panic!(),
            })
            .last()
            .map_or(current, |(_, c)| c);
        if current.x == 0
            || current.y == 0
            || current.x == (grid.width - 1) as i32
            || current.y == (grid.height - 1) as i32
        {
            return false;
        } else if !visited.insert((current, direction)) {
            return true;
        } else {
            direction = next;
            continue;
        }
    }
}
#[aoc(day6, part2)]
fn solver_part2(input: &Input) -> usize {
    let mut grid = input.clone();
    let mut start = Coord::new(0, 0);
    let mut start_direction = Direction::Up;
    for coord in grid.coord_iter() {
        match grid[coord] {
            Square::Guard(d) => {
                start_direction = d;
                start = coord;
                grid[coord] = Square::Visited;
                break;
            }
            _ => continue,
        }
    }
    grid.coord_iter()
        .filter(|coord| {
            if matches!(grid[*coord], Square::Empty) {
                // try put an obstacle in
                let mut grid = grid.clone();
                grid[*coord] = Square::Obstruction;
                is_loop(&mut grid, start, start_direction)
            } else {
                false
            }
        })
        .count()
}

#[aoc(day6, part2, VISITED)]
fn solver_part2_visited(input: &Input) -> usize {
    let mut grid = input.clone();
    let mut start = Coord::new(0, 0);
    let mut start_direction = Direction::Up;
    for coord in grid.coord_iter() {
        match grid[coord] {
            Square::Guard(d) => {
                start_direction = d;
                start = coord;
                grid[coord] = Square::Visited;
                break;
            }
            _ => continue,
        }
    }
    // Run grid to find visited only
    run_grid(&mut grid, start, start_direction);
    // Set starting pos to empty
    grid[start] = Square::Empty;

    grid.coord_iter()
        .filter(|coord| {
            if matches!(grid[*coord], Square::Visited) {
                // try put an obstacle in
                let mut grid = grid.clone();
                grid[*coord] = Square::Obstruction;
                is_loop(&mut grid, start, start_direction)
            } else {
                false
            }
        })
        .count()
}

#[aoc(day6, part2, VISITED_PARALELL)]
fn solver_part2_paralell(input: &Input) -> usize {
    let mut grid = input.clone();
    let mut start = Coord::new(0, 0);
    let mut start_direction = Direction::Up;
    for coord in grid.coord_iter() {
        match grid[coord] {
            Square::Guard(d) => {
                start_direction = d;
                start = coord;
                grid[coord] = Square::Visited;
                break;
            }
            _ => continue,
        }
    }
    // Run grid to find visited only
    run_grid(&mut grid, start, start_direction);
    // Set starting pos to empty
    grid[start] = Square::Empty;

    grid.coord_iter()
        .filter(|coord| matches!(grid[*coord], Square::Visited))
        .par_bridge()
        .filter(|coord| {
            // try put an obstacle in
            let mut grid = grid.clone();
            grid[*coord] = Square::Obstruction;
            is_loop(&mut grid, start, start_direction)
        })
        .count()
}
#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 41)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 6)
    }
}

// AOC 2024
// Day 6 - Part 1 : 4977
//         generator: 113.284µs,
//         runner: 59.554µs

// Day 6 - Part 2 : 1729
//         generator: 95.242µs,
//         runner: 361.642311ms

// Day 6 - Part 2 - VISITED : 1729
//         generator: 82.088µs,
//         runner: 76.265947ms

// Day 6 - Part 2 - VISITED_PARALELL : 1729
//         generator: 83.349µs,
//         runner: 18.59988ms
