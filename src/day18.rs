use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use aoc_tools::grid::Coord;
use aoc_tools::grid::Grid;

type Input = String;

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn find_path(pos: Coord, grid: &Grid<u8>, dist: &mut Grid<u32>, distance: u32) {
    if grid[pos] == 0 && dist[pos] > distance {
        dist[pos] = distance;
        for next in grid.orthogs_coords(&pos).into_iter().flatten() {
            find_path(next.0, grid, dist, distance + 1);
        }
    }
}

#[aoc(day18, part1)]
fn solver_part1(input: &Input) -> u32 {
    let size = 71;
    let bytes = 1024;
    let mut grid = Grid::new(0u8, size, size);
    let mut dist = Grid::new(u32::MAX, size, size);
    for line in input.lines().take(bytes) {
        let (x, y) = line.split_once(',').unwrap();
        let (x, y) = (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap());
        grid[Coord::new(x as i32, y as i32)] = 1;
    }
    // dist.print();

    let pos = Coord::new(0, 0);
    find_path(pos, &grid, &mut dist, 0);
    // dist.print_with_commas();
    // grid.print();
    dist[Coord::new(size as i32 - 1, size as i32 - 1)]
}

#[aoc(day18, part2)]
fn solver_part2(input: &Input) -> String {
    let size = 71;
    let mut bytes = 1024;
    let mut grid = Grid::new(0u8, size, size);

    let mut lines = input.lines();
    for _ in 0..bytes {
        let (x, y) = lines.next().unwrap().split_once(',').unwrap();
        let (x, y) = (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap());
        grid[Coord::new(x as i32, y as i32)] = 1;
    }

    loop {
        bytes += 1;
        let mut dist = Grid::new(u32::MAX, size, size);
        let (x, y) = lines.next().unwrap().split_once(',').unwrap();
        let (x, y) = (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap());
        grid[Coord::new(x as i32, y as i32)] = 1;
        let pos = Coord::new(0, 0);
        find_path(pos, &grid, &mut dist, 0);
        if dist[Coord::new(size as i32 - 1, size as i32 - 1)] == u32::MAX {
            return format!("{x},{y}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 0)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), "")
    }
}
