use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use aoc_tools::grid;
use aoc_tools::grid::Coord;
use aoc_tools::grid::Grid;
use itertools::Itertools;

type Input = String;

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn find_region(
    region: &mut HashMap<Coord, usize>,
    searched: &mut HashSet<Coord>,
    coord: Coord,
    plant: u8,
    grid: &Grid<u8>,
) {
    searched.insert(coord);
    let orthogs = grid
        .orthogs_coords(&coord)
        .into_iter()
        .flatten()
        .filter(|(coord, &plant2)| plant == plant2)
        .collect_vec();
    region.insert(coord, 4 - orthogs.len());
    for (coord, _) in orthogs {
        if !region.contains_key(&coord) {
            find_region(region, searched, coord, plant, grid);
        }
    }
}

#[aoc(day12, part1)]
fn solver_part1(input: &Input) -> usize {
    let width = input.lines().next().unwrap().len();
    let grid = Grid::from_iter(&mut input.lines().flat_map(|l| l.bytes()), width);
    let mut searched = HashSet::default();
    grid.iter()
        .flat_map(|(coord, plant)| {
            if searched.contains(&coord) {
                None
            } else {
                let mut region = HashMap::default();
                find_region(&mut region, &mut searched, coord, *plant, &grid);
                let area = region.len();
                let perimeter = region.values().sum::<usize>();
                // println!("{} : {area} : {perimeter}", *plant as char);
                Some(area * perimeter)
            }
        })
        .sum()
}

#[aoc(day12, part2)]
fn solver_part2(input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 1930)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 0)
    }
}
