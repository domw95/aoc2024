use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
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
        .filter(|(_, &plant2)| plant == plant2)
        .collect_vec();
    region.insert(coord, 4 - orthogs.len());
    for (coord, _) in orthogs {
        if !region.contains_key(&coord) {
            find_region(region, searched, coord, plant, grid);
        }
    }
}
fn find_region_lines(
    region: &mut HashMap<Coord, usize>,
    searched: &mut HashSet<Coord>,
    coord: Coord,
    plant: u8,
    grid: &Grid<u8>,
    lines: &Grid<u32>,
) {
    searched.insert(coord);
    let orthogs = grid
        .orthogs_coords(&coord)
        .into_iter()
        .flatten()
        .filter(|(_, &plant2)| plant == plant2)
        .collect_vec();
    region.insert(coord, lines[coord] as usize);
    for (coord, _) in orthogs {
        if !region.contains_key(&coord) {
            find_region_lines(region, searched, coord, plant, grid, lines);
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
fn solver_part2(input: &Input) -> usize {
    let width = input.lines().next().unwrap().len();
    let grid = Grid::from_iter(&mut input.lines().flat_map(|l| l.bytes()), width);
    let mut vertical = Grid::new(0u8, grid.width, grid.height);
    for row_ind in 0..grid.height {
        // mark the vertical borders
        vertical[Coord::new(0, row_ind as i32)] += 1;
        vertical[Coord::new(grid.width as i32 - 1, row_ind as i32)] += 1;
        for col_ind in 0..(grid.width - 1) {
            if grid[Coord::new(col_ind as i32, row_ind as i32)]
                != grid[Coord::new(col_ind as i32 + 1, row_ind as i32)]
            {
                vertical[Coord::new(col_ind as i32, row_ind as i32)] += 1;
                vertical[Coord::new(col_ind as i32 + 1, row_ind as i32)] += 1;
            }
        }
    }
    // vertical.print();
    let lines = Grid::from_iter(
        &mut grid.iter().map(|(pos, i)| {
            //
            let left = pos.west();
            let right = pos.east();
            let down = pos.south();
            let up = pos.north();
            let left_down = pos.south_west();
            let right_down = pos.south_east();
            // let left_up = pos.north_west();
            let right_up = pos.north_east();

            let left_line = grid.checked_index(&left).map_or(true, |l| l != i)
                && grid.checked_index(&down).map_or(true, |d| {
                    d != i || grid.checked_index(&left_down).map_or(false, |ld| ld == i)
                });
            let right_line = grid.checked_index(&right).map_or(true, |r| r != i)
                && grid.checked_index(&down).map_or(true, |d| {
                    d != i || grid.checked_index(&right_down).map_or(false, |rd| rd == i)
                });
            let down_line = grid.checked_index(&down).map_or(true, |d| d != i)
                && grid.checked_index(&right).map_or(true, |r| {
                    r != i || grid.checked_index(&right_down).map_or(false, |rd| rd == i)
                });
            let up_line = grid.checked_index(&up).map_or(true, |u| u != i)
                && grid.checked_index(&right).map_or(true, |r| {
                    r != i || grid.checked_index(&right_up).map_or(false, |ru| ru == i)
                });
            left_line as u32 + right_line as u32 + down_line as u32 + up_line as u32
        }),
        grid.width,
    );
    let mut searched = HashSet::default();
    grid.iter()
        .flat_map(|(coord, plant)| {
            if searched.contains(&coord) {
                None
            } else {
                let mut region = HashMap::default();
                find_region_lines(&mut region, &mut searched, coord, *plant, &grid, &lines);
                let area = region.len();
                let perimeter = region.values().sum::<usize>();
                // println!("{} : {area} : {perimeter}", *plant as char);
                Some(area * perimeter)
            }
        })
        .sum()

    // let mut horizontal = Grid::new(0u8, grid.width, grid.height);
    // for col_ind in 0..grid.width {
    //     // mark the horizontal borders
    //     horizontal[Coord::new(col_ind as i32, 0)] += 1;
    //     horizontal[Coord::new(col_ind as i32, grid.height as i32 - 1)] += 1;
    //     for row_ind in 0..(grid.height - 1) {
    //         if grid[Coord::new(col_ind as i32, row_ind as i32)]
    //             != grid[Coord::new(col_ind as i32, row_ind as i32 + 1)]
    //         {
    //             horizontal[Coord::new(col_ind as i32, row_ind as i32)] += 1;
    //             horizontal[Coord::new(col_ind as i32, row_ind as i32 + 1)] += 1;
    //         }
    //     }
    // }
    // horizontal.print();
    // 0
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
        assert_eq!(solver_part2(&input_generator(INPUT)), 1206)
    }
}
