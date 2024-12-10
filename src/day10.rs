use std::collections::HashSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use aoc_tools::grid::Coord;
use aoc_tools::grid::Grid;
use fxhash::FxHashSet;

type Input = Grid<u8>;

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    Grid::from_iter(
        &mut input.lines().flat_map(|l| l.bytes().map(|b| b - 48)),
        width,
    )
}

fn find_path(coord: &Coord, next: u8, grid: &Grid<u8>) -> HashSet<Coord> {
    if next == 10 {
        let mut set = HashSet::new();
        set.insert(*coord);
        set
    } else {
        grid.orthogs_coords(coord)
            .iter()
            .flatten()
            .filter(|(_, &i)| i == next)
            .flat_map(|(c, _)| find_path(c, next + 1, grid))
            .collect()
    }
}

#[aoc(day10, part1)]
fn solver_part1(grid: &Input) -> usize {
    grid.iter()
        .filter(|(_, &i)| i == 0)
        .map(|(c, _)| find_path(&c, 1, grid).len())
        .sum()
}

fn find_path_cache(
    coord: &Coord,
    next: u8,
    grid: &Grid<u8>,
    cache: &mut Grid<Option<HashSet<Coord>>>,
) -> HashSet<Coord> {
    if next == 10 {
        let mut set = HashSet::new();
        set.insert(*coord);
        set
    } else if let Some(set) = &cache[*coord] {
        set.clone()
    } else {
        let set: HashSet<Coord> = grid
            .orthogs_coords(coord)
            .iter()
            .flatten()
            .filter(|(_, &i)| i == next)
            .flat_map(|(c, _)| find_path_cache(c, next + 1, grid, cache))
            .collect();
        cache[*coord] = Some(set.clone());
        set
    }
}

#[aoc(day10, part1, CACHE)]
fn solver_part1_cache(grid: &Input) -> usize {
    let mut cache: Grid<Option<HashSet<Coord>>> =
        Grid::from_iter(&mut grid.iter().map(|_| None), grid.width);
    grid.iter()
        .filter(|(_, &i)| i == 0)
        .map(|(c, _)| find_path_cache(&c, 1, grid, &mut cache).len())
        .sum()
}

fn find_path_fxcache(
    coord: &Coord,
    next: u8,
    grid: &Grid<u8>,
    cache: &mut Grid<Option<FxHashSet<Coord>>>,
) -> FxHashSet<Coord> {
    if next == 10 {
        let mut set = FxHashSet::default();
        set.insert(*coord);
        set
    } else if let Some(set) = &cache[*coord] {
        set.clone()
    } else {
        let set: FxHashSet<Coord> = grid
            .orthogs_coords(coord)
            .iter()
            .flatten()
            .filter(|(_, &i)| i == next)
            .flat_map(|(c, _)| find_path_fxcache(c, next + 1, grid, cache))
            .collect();
        cache[*coord] = Some(set.clone());
        set
    }
}

#[aoc(day10, part1, CACHE_FX)]
fn solver_part1_cache_fx(grid: &Input) -> usize {
    let mut cache: Grid<Option<FxHashSet<Coord>>> =
        Grid::from_iter(&mut grid.iter().map(|_| None), grid.width);
    grid.iter()
        .filter(|(_, &i)| i == 0)
        .map(|(c, _)| find_path_fxcache(&c, 1, grid, &mut cache).len())
        .sum()
}

fn find_path_2(coord: &Coord, next: u8, grid: &Grid<u8>) -> usize {
    if next == 10 {
        1
    } else {
        grid.orthogs_coords(coord)
            .iter()
            .flatten()
            .filter(|(_, &i)| i == next)
            .map(|(c, _)| find_path_2(c, next + 1, grid))
            .sum()
    }
}

#[aoc(day10, part2)]
fn solver_part2(grid: &Input) -> usize {
    grid.iter()
        .filter(|(_, &i)| i == 0)
        .map(|(c, _)| find_path_2(&c, 1, grid))
        .sum()
}

fn find_path_2_cache(
    coord: &Coord,
    next: u8,
    grid: &Grid<u8>,
    cache: &mut Grid<Option<usize>>,
) -> usize {
    if next == 10 {
        1
    } else if let Some(v) = cache[*coord] {
        v
    } else {
        let v = grid
            .orthogs_coords(coord)
            .iter()
            .flatten()
            .filter(|(_, &i)| i == next)
            .map(|(c, _)| find_path_2_cache(c, next + 1, grid, cache))
            .sum();
        cache[*coord] = Some(v);
        v
    }
}

#[aoc(day10, part2, CACHE)]
fn solver_part2_cache(grid: &Input) -> usize {
    let mut cache: Grid<Option<usize>> =
        Grid::from_iter(&mut grid.iter().map(|_| None), grid.width);
    grid.iter()
        .filter(|(_, &i)| i == 0)
        .map(|(c, _)| find_path_2_cache(&c, 1, grid, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day10::solver_part1_cache;
    use crate::day10::solver_part1_cache_fx;
    use crate::day10::solver_part2_cache;

    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 36)
    }
    #[test]
    fn part1_cache() {
        assert_eq!(solver_part1_cache(&input_generator(INPUT)), 36)
    }

    #[test]
    fn part1_cache_fx() {
        assert_eq!(solver_part1_cache_fx(&input_generator(INPUT)), 36)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 81)
    }

    #[test]
    fn part2_cache() {
        assert_eq!(solver_part2_cache(&input_generator(INPUT)), 81)
    }
}
