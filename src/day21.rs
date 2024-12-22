use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use fxhash::FxHashMap;

type Input = String;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Button {
    Left,
    Up,
    Right,
    Down,
    A,
}

impl Button {
    fn steps(&self, to: &Self) -> Vec<Vec<Button>> {
        match (self, to) {
            (Button::Left, Button::Up) => vec![vec![Button::Right, Button::Up, Button::A]],
            (Button::Left, Button::Right) => vec![vec![Button::Right, Button::Right, Button::A]],
            (Button::Left, Button::Down) => vec![vec![Button::Right, Button::A]],
            (Button::Left, Button::A) => {
                vec![vec![Button::Right, Button::Right, Button::Up, Button::A]]
            }
            (Button::Up, Button::Left) => vec![vec![Button::Down, Button::Left, Button::A]],
            (Button::Up, Button::Right) => vec![
                vec![Button::Down, Button::Right, Button::A],
                vec![Button::Right, Button::Down, Button::A],
            ],
            (Button::Up, Button::Down) => vec![vec![Button::Down, Button::A]],
            (Button::Up, Button::A) => vec![vec![Button::Right, Button::A]],
            (Button::Right, Button::Left) => vec![vec![Button::Left, Button::Left, Button::A]],
            (Button::Right, Button::Up) => vec![
                vec![Button::Left, Button::Up, Button::A],
                vec![Button::Up, Button::Left, Button::A],
            ],
            (Button::Right, Button::Down) => vec![vec![Button::Left, Button::A]],
            (Button::Right, Button::A) => vec![vec![Button::Up, Button::A]],
            (Button::Down, Button::Left) => vec![vec![Button::Left, Button::A]],
            (Button::Down, Button::Up) => vec![vec![Button::Up, Button::A]],
            (Button::Down, Button::Right) => vec![vec![Button::Right, Button::A]],
            (Button::Down, Button::A) => vec![
                vec![Button::Up, Button::Right, Button::A],
                vec![Button::Right, Button::Up, Button::A],
            ],
            (Button::A, Button::Left) => {
                vec![vec![Button::Down, Button::Left, Button::Left, Button::A]]
            }
            (Button::A, Button::Up) => vec![vec![Button::Left, Button::A]],
            (Button::A, Button::Right) => vec![vec![Button::Down, Button::A]],
            (Button::A, Button::Down) => vec![
                vec![Button::Left, Button::Down, Button::A],
                vec![Button::Down, Button::Left, Button::A],
            ],
            _ => vec![vec![Button::A]],
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Key {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    Zero,
}

impl Key {
    fn pos(&self) -> (i8, i8) {
        self.into()
    }

    fn dist(&self, other: &Self) -> (i8, i8) {
        let a = self.pos();
        let b = other.pos();
        (b.0 - a.0, b.1 - a.1)
    }

    fn steps(&self, other: &Self) -> Vec<Vec<Button>> {
        let (x, y) = self.dist(other);
        let mut y_vec = if y < 0 {
            vec![Button::Down; y.unsigned_abs() as usize]
        } else {
            vec![Button::Up; y as usize]
        };
        let mut x_vec = if x < 0 {
            vec![Button::Left; x.unsigned_abs() as usize]
        } else {
            vec![Button::Right; x as usize]
        };
        let mut vecs = match (self, other) {
            (Key::A | Key::Zero, Key::One | Key::Four | Key::Seven) => {
                // Must go up then left
                y_vec.append(&mut x_vec);
                vec![y_vec]
            }
            (Key::One | Key::Four | Key::Seven, Key::A | Key::Zero) => {
                // Must go right then down
                x_vec.append(&mut y_vec);
                vec![x_vec]
            }
            _ => {
                // Can go either way
                if x == 0 {
                    vec![y_vec]
                } else if y == 0 {
                    vec![x_vec]
                } else {
                    let mut v1 = y_vec.clone();
                    let mut v2 = x_vec.clone();
                    v1.append(&mut x_vec);
                    v2.append(&mut y_vec);
                    vec![v1, v2]
                }
            }
        };
        for v in &mut vecs {
            v.push(Button::A);
        }
        vecs
    }
}

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        match value {
            b'0' => Key::Zero,
            b'1' => Key::One,
            b'2' => Key::Two,
            b'3' => Key::Three,
            b'4' => Key::Four,
            b'5' => Key::Five,
            b'6' => Key::Six,
            b'7' => Key::Seven,
            b'8' => Key::Eight,
            b'9' => Key::Nine,
            _ => Key::A,
        }
    }
}

impl From<&Key> for (i8, i8) {
    fn from(value: &Key) -> Self {
        match value {
            Key::One => (0, 0),
            Key::Two => (1, 0),
            Key::Three => (2, 0),
            Key::Four => (0, 1),
            Key::Five => (1, 1),
            Key::Six => (2, 1),
            Key::Seven => (0, 2),
            Key::Eight => (1, 2),
            Key::Nine => (2, 2),
            Key::A => (2, -1),
            Key::Zero => (1, -1),
        }
    }
}

#[aoc_generator(day21)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn find_shortest<const N: usize>(
    paths: &[Vec<Button>],
    depth: usize,
    state: [Button; N],
) -> (usize, [Button; N]) {
    if depth == N {
        (paths[0].len(), state)
    } else {
        paths
            .iter()
            .map(|path| {
                path.iter()
                    .fold(
                        ((0usize, state), state[depth]),
                        |((len, mut state), prev), b| {
                            state[depth] = *b;
                            let (l, state) = find_shortest(&prev.steps(b), depth + 1, state);
                            ((len + l, state), *b)
                        },
                    )
                    .0
            })
            .min_by_key(|v| v.0)
            .unwrap()
    }
}

fn find_shortest_faster<const N: usize>(paths: &[Vec<Button>], depth: usize) -> usize {
    if depth == N {
        paths[0].len()
    } else {
        paths
            .iter()
            .map(|path| {
                path.iter()
                    .fold((0usize, Button::A), |(len, prev), b| {
                        let l = find_shortest_faster::<N>(&prev.steps(b), depth + 1);
                        (len + l, *b)
                    })
                    .0
            })
            .min()
            .unwrap()
    }
}

// #[derive(Default)]
struct Cache<const N: usize> {
    maps: [FxHashMap<(Button, Button), usize>; N],
}

impl<const N: usize> Cache<N> {
    fn new() -> Self {
        Cache {
            maps: core::array::from_fn(|_| FxHashMap::default()),
        }
    }

    fn insert(&mut self, from: Button, to: Button, depth: usize, value: usize) {
        self.maps[depth].insert((from, to), value);
    }

    fn get(&self, from: Button, to: Button, depth: usize) -> Option<&usize> {
        self.maps[depth].get(&(from, to))
    }
}

struct GridCache<const N: usize> {
    maps: [[Option<usize>; 25]; N],
}

impl<const N: usize> GridCache<N> {
    fn new() -> Self {
        GridCache {
            maps: core::array::from_fn(|_| [None; 25]),
        }
    }

    fn insert(&mut self, from: Button, to: Button, depth: usize, value: usize) {
        self.maps[depth][GridCache::<N>::get_index_2(from, to)] = Some(value);
    }

    fn get(&self, from: Button, to: Button, depth: usize) -> Option<usize> {
        self.maps[depth][GridCache::<N>::get_index_2(from, to)]
    }

    fn get_index_2(from: Button, to: Button) -> usize {
        from as usize * 5 + to as usize
    }

    fn _get_index(from: Button, to: Button) -> usize {
        match (from, to) {
            (Button::Left, Button::Left) => 0,
            (Button::Left, Button::Up) => 1,
            (Button::Left, Button::Right) => 2,
            (Button::Left, Button::Down) => 3,
            (Button::Left, Button::A) => 4,
            (Button::Up, Button::Left) => 5,
            (Button::Up, Button::Up) => 6,
            (Button::Up, Button::Right) => 7,
            (Button::Up, Button::Down) => 8,
            (Button::Up, Button::A) => 9,
            (Button::Right, Button::Left) => 10,
            (Button::Right, Button::Up) => 11,
            (Button::Right, Button::Right) => 12,
            (Button::Right, Button::Down) => 13,
            (Button::Right, Button::A) => 14,
            (Button::Down, Button::Left) => 15,
            (Button::Down, Button::Up) => 16,
            (Button::Down, Button::Right) => 17,
            (Button::Down, Button::Down) => 18,
            (Button::Down, Button::A) => 19,
            (Button::A, Button::Left) => 20,
            (Button::A, Button::Up) => 21,
            (Button::A, Button::Right) => 22,
            (Button::A, Button::Down) => 23,
            (Button::A, Button::A) => 24,
        }
    }
}

struct FlatCache {
    maps: [Option<usize>; 625],
}

impl FlatCache {
    fn new() -> Self {
        FlatCache {
            maps: core::array::from_fn(|_| None),
        }
    }

    fn insert(&mut self, from: Button, to: Button, depth: usize, value: usize) {
        self.maps[FlatCache::get_index(depth, from, to)] = Some(value);
    }

    fn get(&self, from: Button, to: Button, depth: usize) -> Option<usize> {
        self.maps[FlatCache::get_index(depth, from, to)]
    }

    fn get_index(depth: usize, from: Button, to: Button) -> usize {
        (depth * 25) + (from as usize * 5) + to as usize
    }
}
fn find_shortest_cached<const N: usize>(
    paths: &[Vec<Button>],
    depth: usize,
    cache: &mut Cache<N>,
) -> usize {
    if depth == N {
        // println!("End");
        paths[0].len()
    } else {
        paths
            .iter()
            .map(|path| {
                path.iter()
                    .fold((0usize, Button::A), |(len, prev), b| {
                        let l = if let Some(value) = cache.get(prev, *b, depth) {
                            // println!("Cache hit {}", value);
                            *value
                        } else {
                            let l = find_shortest_cached(&prev.steps(b), depth + 1, cache);
                            cache.insert(prev, *b, depth, l);
                            l
                        };

                        (len + l, *b)
                    })
                    .0
            })
            .min()
            .unwrap()
    }
}

fn find_shortest_grid_cached<const N: usize>(
    paths: &[Vec<Button>],
    depth: usize,
    cache: &mut GridCache<N>,
) -> usize {
    if depth == N {
        // println!("End");
        paths[0].len()
    } else {
        paths
            .iter()
            .map(|path| {
                path.iter()
                    .fold((0usize, Button::A), |(len, prev), b| {
                        let l = if let Some(value) = cache.get(prev, *b, depth) {
                            // println!("Cache hit {}", value);
                            value
                        } else {
                            let l = find_shortest_grid_cached(&prev.steps(b), depth + 1, cache);
                            cache.insert(prev, *b, depth, l);
                            l
                        };

                        (len + l, *b)
                    })
                    .0
            })
            .min()
            .unwrap()
    }
}

fn find_shortest_flat_cached<const N: usize>(
    paths: &[Vec<Button>],
    depth: usize,
    cache: &mut FlatCache,
) -> usize {
    if depth == N {
        // println!("End");
        paths[0].len()
    } else {
        paths
            .iter()
            .map(|path| {
                path.iter()
                    .fold((0usize, Button::A), |(len, prev), b| {
                        let l = if let Some(value) = cache.get(prev, *b, depth) {
                            // println!("Cache hit {}", value);
                            value
                        } else {
                            let l =
                                find_shortest_flat_cached::<N>(&prev.steps(b), depth + 1, cache);
                            cache.insert(prev, *b, depth, l);
                            l
                        };

                        (len + l, *b)
                    })
                    .0
            })
            .min()
            .unwrap()
    }
}

#[aoc(day21, part1)]
fn solver_part1(input: &Input) -> usize {
    let mut state = [Button::A; 2];
    input
        .lines()
        .map(|line| {
            let value = line[0..3].parse::<usize>().unwrap();

            let len = line
                .bytes()
                .map(Key::from)
                .fold((0usize, Key::A), |(len, prev), k| {
                    let l;
                    (l, state) = find_shortest(&prev.steps(&k), 0, state);
                    (len + l, k)
                })
                .0;
            // dbg!(value, len);
            value * len
        })
        .sum()
}

#[aoc(day21, part1, FASTER)]
fn solver_part1_faster(input: &Input) -> usize {
    input
        .lines()
        .map(|line| {
            let value = line[0..3].parse::<usize>().unwrap();

            let len = line
                .bytes()
                .map(Key::from)
                .fold((0usize, Key::A), |(len, prev), k| {
                    let l = find_shortest_faster::<2>(&prev.steps(&k), 0);
                    (len + l, k)
                })
                .0;
            // dbg!(value, len);
            value * len
        })
        .sum()
}

#[aoc(day21, part1, Cache)]
fn solver_part1_cache(input: &Input) -> usize {
    const SIZE: usize = 2;
    input
        .lines()
        .map(|line| {
            let value = line[0..3].parse::<usize>().unwrap();

            let len = line
                .bytes()
                .map(Key::from)
                .fold((0usize, Key::A), |(len, prev), k| {
                    let l = find_shortest_grid_cached(
                        &prev.steps(&k),
                        0,
                        &mut GridCache::<SIZE>::new(),
                    );
                    (len + l, k)
                })
                .0;
            // dbg!(value, len);
            value * len
        })
        .sum()
}

#[aoc(day21, part2)]
fn solver_part2(input: &Input) -> usize {
    const SIZE: usize = 25;
    input
        .lines()
        .map(|line| {
            let value = line[0..3].parse::<usize>().unwrap();

            let len = line
                .bytes()
                .map(Key::from)
                .fold((0usize, Key::A), |(len, prev), k| {
                    let l = find_shortest_cached(&prev.steps(&k), 0, &mut Cache::<SIZE>::new());
                    (len + l, k)
                })
                .0;
            // dbg!(value, len);
            value * len
        })
        .sum()
}

#[aoc(day21, part2, GRID_CACHE)]
fn solver_part2_grid_cache(input: &Input) -> usize {
    const SIZE: usize = 25;
    input
        .lines()
        .map(|line| {
            let value = line[0..3].parse::<usize>().unwrap();

            let len = line
                .bytes()
                .map(Key::from)
                .fold((0usize, Key::A), |(len, prev), k| {
                    let l = find_shortest_grid_cached(
                        &prev.steps(&k),
                        0,
                        &mut GridCache::<SIZE>::new(),
                    );
                    (len + l, k)
                })
                .0;
            // dbg!(value, len);
            value * len
        })
        .sum()
}

#[aoc(day21, part2, FLAT_CACHE)]
fn solver_part2_flat_cache(input: &Input) -> usize {
    const SIZE: usize = 25;
    input
        .lines()
        .map(|line| {
            let value = line[0..3].parse::<usize>().unwrap();

            let len = line
                .bytes()
                .map(Key::from)
                .fold((0usize, Key::A), |(len, prev), k| {
                    let l = find_shortest_flat_cached::<SIZE>(
                        &prev.steps(&k),
                        0,
                        &mut FlatCache::new(),
                    );
                    (len + l, k)
                })
                .0;
            // dbg!(value, len);
            value * len
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 126384)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 0)
    }
}
