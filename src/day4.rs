use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Index;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Input = Grid<char>;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    fn north_east(&self) -> Coord {
        Coord::new(self.x + 1, self.y - 1)
    }

    fn north_west(&self) -> Coord {
        Coord::new(self.x - 1, self.y - 1)
    }

    fn south_east(&self) -> Coord {
        Coord::new(self.x + 1, self.y + 1)
    }

    fn south_west(&self) -> Coord {
        Coord::new(self.x - 1, self.y + 1)
    }

    fn diags(&self) -> [Coord; 4] {
        [
            self.north_east(),
            self.south_east(),
            self.south_west(),
            self.north_west(),
        ]
    }

    fn step_return(&mut self, other: &Self) -> Self {
        let prev = self.clone();
        *self += *other;
        prev
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        *self + *rhs
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
#[derive(Debug, Clone)]
struct Grid<T> {
    items: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.items[self.width * index.y as usize + index.x as usize]
    }
}

impl<T> Grid<T> {
    fn from_iter(iter: &mut dyn Iterator<Item = T>, width: usize) -> Self {
        let items: Vec<T> = iter.collect();
        let height = items.len() / width;
        Grid {
            items,
            width,
            height,
        }
    }

    fn bounds_check(&self, coord: &Coord) -> bool {
        coord.x < self.width as i32 && coord.y < self.height as i32 && coord.x >= 0 && coord.y >= 0
    }

    fn coord_iter(&self) -> std::iter::Map<std::ops::Range<usize>, impl FnMut(usize) -> Coord> {
        let width = self.width;
        (0..self.items.len()).map(move |i| Coord::new((i % width) as i32, (i / width) as i32))
    }

    fn diags(&self, coord: &Coord) -> [Option<&T>; 4] {
        coord.diags().map(|c| self.checked_index(&c))
    }

    fn checked_index(&self, coord: &Coord) -> Option<&T> {
        if self.bounds_check(coord) {
            Some(&self[*coord])
        } else {
            None
        }
    }
}

impl<'a, T> Grid<T> {
    fn stride_iter(&'a self, start: Coord, stride: Coord) -> GridLineIter<'a, T> {
        GridLineIter {
            grid: self,
            coord: start,
            stride,
        }
    }
    fn north_iter(&'a self, start: Coord) -> GridLineIter<'a, T> {
        self.stride_iter(start, Coord::new(0, -1))
    }
    fn east_iter(&'a self, start: Coord) -> GridLineIter<'a, T> {
        self.stride_iter(start, Coord::new(1, 0))
    }

    fn south_iter(&'a self, start: Coord) -> GridLineIter<'a, T> {
        self.stride_iter(start, Coord::new(0, 1))
    }

    fn west_iter(&'a self, start: Coord) -> GridLineIter<'a, T> {
        self.stride_iter(start, Coord::new(-1, 0))
    }

    fn north_east_iter(&'a self, start: Coord) -> GridLineIter<'a, T> {
        self.stride_iter(start, Coord::new(1, -1))
    }

    fn north_west_iter(&'a self, start: Coord) -> GridLineIter<'a, T> {
        self.stride_iter(start, Coord::new(-1, -1))
    }

    fn south_east_iter(&'a self, start: Coord) -> GridLineIter<'a, T> {
        self.stride_iter(start, Coord::new(1, 1))
    }

    fn south_west_iter(&'a self, start: Coord) -> GridLineIter<'a, T> {
        self.stride_iter(start, Coord::new(-1, 1))
    }
    fn orthogonal_direction_iterators(&'a self, start: Coord) -> [GridLineIter<'a, T>; 4] {
        [
            self.north_iter(start),
            self.east_iter(start),
            self.south_iter(start),
            self.west_iter(start),
        ]
    }
    fn diagonal_direction_iterators(&'a self, start: Coord) -> [GridLineIter<'a, T>; 4] {
        [
            self.north_east_iter(start),
            self.south_east_iter(start),
            self.south_west_iter(start),
            self.north_west_iter(start),
        ]
    }
    fn all_direction_iterators(&'a self, start: Coord) -> [GridLineIter<'a, T>; 8] {
        [
            self.north_iter(start),
            self.east_iter(start),
            self.south_iter(start),
            self.west_iter(start),
            self.north_east_iter(start),
            self.south_east_iter(start),
            self.south_west_iter(start),
            self.north_west_iter(start),
        ]
    }
}

struct GridLineIter<'a, T> {
    grid: &'a Grid<T>,
    coord: Coord,
    stride: Coord,
}

impl<'a, T> Iterator for GridLineIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.bounds_check(&self.coord) {
            Some(&self.grid[self.coord.step_return(&self.stride)])
        } else {
            None
        }
    }
}

struct GridCoordIter<'a, T> {
    grid: &'a Grid<T>,
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    Grid::from_iter(&mut input.lines().flat_map(|l| l.chars()), width)
}

#[aoc(day4, part1)]
fn solver_part1(grid: &Input) -> usize {
    grid.coord_iter()
        .flat_map(|coord| {
            grid.all_direction_iterators(coord)
                .into_iter()
                .map(|iter| iter.take(4).collect::<String>() == "XMAS")
                .filter(|v| *v)
                .collect::<Vec<bool>>()
        })
        .count()
}

#[aoc(day4, part1, FAST)]
fn solver_part1_fast(grid: &Input) -> usize {
    grid.coord_iter()
        .flat_map(|coord| {
            grid.all_direction_iterators(coord)
                .into_iter()
                .map(|mut iter| {
                    matches!(iter.next(), Some('X'))
                        && matches!(iter.next(), Some('M'))
                        && matches!(iter.next(), Some('A'))
                        && matches!(iter.next(), Some('S'))
                })
                .filter(|v| *v)
                .collect::<Vec<bool>>()
        })
        .count()
}

#[aoc(day4, part2)]
fn solver_part2(grid: &Input) -> usize {
    grid.coord_iter()
        .filter(|coord| {
            if grid[*coord] == 'A' {
                let mut diags = grid
                    .diags(coord)
                    .into_iter()
                    .flatten()
                    .copied()
                    .collect_vec();
                (0..4).any(|_| {
                    if diags == vec!['M', 'M', 'S', 'S'] {
                        true
                    } else {
                        diags.rotate_left(1);
                        false
                    }
                })
            } else {
                false
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 18)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 9)
    }
}
