use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use aoc_tools::grid::Coord;
use aoc_tools::grid::Grid;
use rayon::prelude::*;

type Input = String;

#[aoc_generator(day20)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn parse_input(input: &str) -> Grid<u8> {
    let width = input.lines().next().unwrap().len();
    Grid::from_iter(&mut input.lines().flat_map(|l| l.bytes()), width)
}

fn find_route(grid: &Grid<u8>, start: Coord) -> (Grid<u32>, Vec<Coord>) {
    let mut steps = Grid::new(u32::MAX, grid.width, grid.height);
    steps[start] = 0;
    let mut nodes = vec![start];
    let mut path = vec![start];
    let mut count = 0;
    loop {
        count += 1;
        let mut new_nodes = Vec::new();
        for node in nodes {
            for next in node.orthogs() {
                if let Some(cell) = grid.checked_index(&next) {
                    if *cell == b'.' && steps[next] > count {
                        steps[next] = count;
                        new_nodes.push(next);
                        path.push(next);
                    }
                }
            }
        }
        if new_nodes.is_empty() {
            break;
        } else {
            nodes = new_nodes;
        }
    }
    (steps, path)
}

fn find_route_faster(grid: &Grid<u8>, start: Coord) -> (Grid<u32>, Vec<Coord>) {
    let mut steps = Grid::new(u32::MAX, grid.width, grid.height);
    steps[start] = 0;
    let mut next = start;
    let mut path = vec![start];
    let mut count = 0;
    'outer: loop {
        count += 1;
        for node in next.orthogs() {
            if let Some(cell) = grid.checked_index(&node) {
                if *cell == b'.' && steps[node] == u32::MAX {
                    steps[node] = count;
                    next = node;
                    path.push(node);
                    continue 'outer;
                }
            }
        }

        break;
    }
    (steps, path)
}

fn find_shortcuts(steps: &Grid<u32>, path: &[Coord]) -> Vec<u32> {
    let mut saved = Vec::new();
    for coord in path {
        let start = steps[*coord];
        for next in coord.orthog_steps(2).into_iter() {
            if let Some(&dist) = steps.checked_index(&next) {
                if dist != u32::MAX && dist > start && (dist - start) > 2 {
                    saved.push(dist - start - 2);
                }
            }
        }
    }
    saved
}

fn find_shortcuts_100(steps: &Grid<u32>, path: &[Coord]) -> usize {
    let mut count = 0;
    for coord in path {
        let start = steps[*coord];
        for next in coord.orthog_steps(2).into_iter() {
            if let Some(&dist) = steps.checked_index(&next) {
                if dist != u32::MAX && dist > start && (dist - start) >= 102 {
                    count += 1
                }
            }
        }
    }
    count
}

fn find_shortcuts_1(_steps: &Grid<u32>, path: &[Coord]) -> usize {
    let mut count = 0;
    for i in 0..path.len() {
        let coord = path[i];
        for (j, next) in path[i..].iter().enumerate() {
            let distance = coord.rectilinear_distance(next);
            if distance <= 2 {
                let gained = j;
                if gained > distance as usize && gained - distance as usize >= 100 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn find_shortcuts_2(_steps: &Grid<u32>, path: &[Coord]) -> usize {
    let mut count = 0;
    for i in 0..path.len() {
        let coord = path[i];
        for (j, next) in path[i..].iter().enumerate() {
            let distance = coord.rectilinear_distance(next);
            if distance <= 20 {
                let gained = j;
                // dbg!(i, j, distance, gained);
                if gained > distance as usize && gained - distance as usize >= 100 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn find_shortcuts_2_full_slice(_steps: &Grid<u32>, path: &[Coord]) -> usize {
    path.iter()
        .enumerate()
        .map(|(i, coord)| {
            path[i..]
                .iter()
                .enumerate()
                .filter(|(gained, next)| {
                    let distance = coord.rectilinear_distance(next);
                    distance <= 20
                        && *gained > distance as usize
                        && gained - distance as usize >= 100
                })
                .count()
        })
        .sum()
}

fn find_shortcuts_2_parallel(_steps: &Grid<u32>, path: &[Coord]) -> usize {
    path.into_par_iter()
        .enumerate()
        .map(|(i, coord)| {
            let mut count = 0;
            for (j, next) in path[i..].iter().enumerate() {
                let distance = coord.rectilinear_distance(next);
                if distance <= 20 {
                    let gained = j;
                    // dbg!(i, j, distance, gained);
                    if gained > distance as usize && gained - distance as usize >= 100 {
                        count += 1;
                    }
                }
            }
            count
        })
        .sum()
}

fn find_shortcuts_2_parallel_2(_steps: &Grid<u32>, path: &[Coord]) -> usize {
    path[0..(path.len() - 102)]
        .into_par_iter()
        .enumerate()
        .map(|(i, coord)| {
            path[(i + 102)..]
                .iter()
                .enumerate()
                .filter(|(j, next)| {
                    let distance = coord.rectilinear_distance(next) as usize;
                    distance <= 20 && (*j + 2) >= distance
                })
                .count()
        })
        .sum()
}

fn find_shortcuts_2_worse(_steps: &Grid<u32>, path: &[Coord]) -> usize {
    let mut count = 0;
    for i in 0..(path.len() - 102) {
        let coord = path[i];
        let start = i + 100;
        for (j, next) in path[start..].iter().enumerate() {
            let distance = coord.rectilinear_distance(next);
            if distance <= 20 && j >= distance as usize {
                count += 1;
            }
        }
    }
    count
}

fn find_shortcuts_2_slice(_steps: &Grid<u32>, path: &[Coord]) -> usize {
    let mut count = 0;
    // assert!(path.len() > 102);
    let end = path.len() - 102;

    for (i, coord) in path[0..end].iter().enumerate() {
        let start = i + 102;
        // assert!(start < path.len());
        for (j, next) in path[start..].iter().enumerate() {
            let distance = coord.rectilinear_distance(next) as usize;
            if distance <= 20 && j + 2 >= distance {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day20, part1)]
fn solver_part1(input: &Input) -> usize {
    let mut grid = parse_input(input);
    let mut start = Coord::new(0, 0);
    for c in grid.coord_iter() {
        if grid[c] == b'S' {
            start = c;
            grid[c] = b'.';
        }
        if grid[c] == b'E' {
            grid[c] = b'.';
        }
    }
    // grid.print_func(|&b| format!("{}", b as char));
    let (steps, path) = find_route(&grid, start);

    find_shortcuts(&steps, &path)
        .iter()
        .filter(|&&c| c >= 100)
        .count()
}

#[aoc(day20, part1, FASTER)]
fn solver_part1_faster(input: &Input) -> usize {
    let mut grid = parse_input(input);
    let mut start = Coord::new(0, 0);
    for c in grid.coord_iter() {
        if grid[c] == b'S' {
            start = c;
            grid[c] = b'.';
        }
        if grid[c] == b'E' {
            grid[c] = b'.';
        }
    }
    // grid.print_func(|&b| format!("{}", b as char));
    let (steps, path) = find_route_faster(&grid, start);
    find_shortcuts_100(&steps, &path)
}

#[aoc(day20, part1, ALT)]
fn solver_part1_alt(input: &Input) -> usize {
    let mut grid = parse_input(input);
    let mut start = Coord::new(0, 0);
    for c in grid.coord_iter() {
        if grid[c] == b'S' {
            start = c;
            grid[c] = b'.';
        }
        if grid[c] == b'E' {
            grid[c] = b'.';
        }
    }
    // grid.print_func(|&b| format!("{}", b as char));
    let (steps, path) = find_route_faster(&grid, start);
    find_shortcuts_1(&steps, &path)
}

#[aoc(day20, part2)]
fn solver_part2(input: &Input) -> usize {
    let mut grid = parse_input(input);
    let mut start = Coord::new(0, 0);
    for c in grid.coord_iter() {
        if grid[c] == b'S' {
            start = c;
            grid[c] = b'.';
        }
        if grid[c] == b'E' {
            grid[c] = b'.';
        }
    }
    // grid.print_func(|&b| format!("{}", b as char));
    let (steps, path) = find_route_faster(&grid, start);
    find_shortcuts_2(&steps, &path)

    // steps.print_func(|s| {
    //     if *s == u32::MAX {
    //         "####".to_string()
    //     } else {
    //         format!(" {:2} ", s)
    //     }
    // });
    // count
}

#[aoc(day20, part2, PARALLEL)]
fn solver_part2_parallel(input: &Input) -> usize {
    let mut grid = parse_input(input);
    let mut start = Coord::new(0, 0);
    for c in grid.coord_iter() {
        if grid[c] == b'S' {
            start = c;
            grid[c] = b'.';
        }
        if grid[c] == b'E' {
            grid[c] = b'.';
        }
    }
    // grid.print_func(|&b| format!("{}", b as char));
    let (steps, path) = find_route_faster(&grid, start);
    find_shortcuts_2_parallel(&steps, &path)
}

#[aoc(day20, part2, PARALLEL_2)]
fn solver_part2_parallel_2(input: &Input) -> usize {
    let mut grid = parse_input(input);
    let mut start = Coord::new(0, 0);
    for c in grid.coord_iter() {
        if grid[c] == b'S' {
            start = c;
            grid[c] = b'.';
        }
        if grid[c] == b'E' {
            grid[c] = b'.';
        }
    }
    // grid.print_func(|&b| format!("{}", b as char));
    let (steps, path) = find_route_faster(&grid, start);
    find_shortcuts_2_parallel_2(&steps, &path)
}

#[aoc(day20, part2, FULL_SLICE)]
fn solver_part2_full_slice(input: &Input) -> usize {
    let mut grid = parse_input(input);
    let mut start = Coord::new(0, 0);
    for c in grid.coord_iter() {
        if grid[c] == b'S' {
            start = c;
            grid[c] = b'.';
        }
        if grid[c] == b'E' {
            grid[c] = b'.';
        }
    }
    // grid.print_func(|&b| format!("{}", b as char));
    let (steps, path) = find_route_faster(&grid, start);
    find_shortcuts_2_full_slice(&steps, &path)
}

#[aoc(day20, part2, WORSE)]
fn solver_part2_worse(input: &Input) -> usize {
    let mut grid = parse_input(input);
    let mut start = Coord::new(0, 0);
    for c in grid.coord_iter() {
        if grid[c] == b'S' {
            start = c;
            grid[c] = b'.';
        }
        if grid[c] == b'E' {
            grid[c] = b'.';
        }
    }
    let (steps, path) = find_route_faster(&grid, start);
    find_shortcuts_2_worse(&steps, &path)
}

#[aoc(day20, part2, SLICE)]
fn solver_part2_slice(input: &Input) -> usize {
    let mut grid = parse_input(input);
    let mut start = Coord::new(0, 0);
    for c in grid.coord_iter() {
        if grid[c] == b'S' {
            start = c;
            grid[c] = b'.';
        }
        if grid[c] == b'E' {
            grid[c] = b'.';
        }
    }
    let (steps, path) = find_route_faster(&grid, start);
    find_shortcuts_2_slice(&steps, &path)
}

#[cfg(test)]
mod tests {
    use crate::day20::solver_part1_faster;

    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 0)
    }

    #[test]
    fn part1_2() {
        assert_eq!(solver_part1_faster(&input_generator(INPUT)), 0)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 0)
    }
}
