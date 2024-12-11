use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Input = String;

#[derive(Debug, Clone, Copy)]
enum Block {
    Empty,
    Id(u32),
}

impl Block {
    fn _value(&self) -> u32 {
        match self {
            Block::Empty => 0,
            Block::Id(v) => *v,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Chunk {
    Empty(usize),
    Id(usize, u32),
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn create_blocks(str: &str) -> Vec<Block> {
    str.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let v = c as u8 - 48;
            if i % 2 == 0 {
                vec![Block::Id(i as u32 / 2); v as usize]
            } else {
                vec![Block::Empty; v as usize]
            }
        })
        .collect()
}

fn swap_blocks(fs: &mut [Block]) {
    let mut head = 0;
    let mut tail = fs.len() - 1;
    'outer: loop {
        // got to next free block
        while matches!(fs[head], Block::Id(_)) {
            head += 1;
            if head >= tail {
                break 'outer;
            }
        }
        // go to next filled block
        while matches!(fs[tail], Block::Empty) {
            tail -= 1;
            if head >= tail {
                break 'outer;
            }
        }
        fs.swap(head, tail);

        head += 1;
        tail -= 1;
    }
}

fn checksum_blocks(fs: &[Block]) -> usize {
    fs.iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            Block::Empty => None,
            Block::Id(v) => Some(*v as usize * i),
        })
        .sum()
}

#[aoc(day9, part1)]
fn solver_part1(input: &Input) -> usize {
    let mut fs = create_blocks(input);
    swap_blocks(&mut fs);
    checksum_blocks(&fs)
}

fn _next_tail(tail: &mut usize, fs: &[Block]) {
    *tail -= 1;
    while matches!(fs[*tail], Block::Empty) {
        *tail -= 1;
    }
}
#[aoc(day9, part1, INLINE)]
fn solver_part1_inline(input: &Input) -> usize {
    let fs = input.chars().map(|c| c as u8 - 48).collect_vec();
    let mut head = 0;
    let mut tail = if fs.len() % 2 == 0 {
        fs.len() - 2
    } else {
        fs.len() - 1
    };
    let mut head_count = fs[0];
    let mut tail_count = fs[tail];
    let mut index = 0;

    let mut checksum = 0;
    let mut file = true;

    'outer: loop {
        if file {
            for _ in 0..head_count {
                checksum += index * head / 2;
                index += 1;
            }
        } else {
            for _ in 0..head_count {
                checksum += index * tail / 2;
                index += 1;

                tail_count -= 1;
                while tail_count == 0 {
                    tail -= 2;
                    if tail <= head {
                        break 'outer;
                    }
                    tail_count = fs[tail]
                }
            }
        }
        head += 1;
        head_count = fs[head];
        file = !file;
        if head == tail {
            for _ in 0..tail_count {
                checksum += index * tail / 2;
                index += 1
            }
            break;
        }
    }

    checksum
}

fn create_chunks(str: &str) -> Vec<Chunk> {
    str.chars()
        .enumerate()
        .filter_map(|(i, c)| {
            let v = c as u8 - 48;
            if i % 2 == 0 {
                Some(Chunk::Id(v as usize, i as u32 / 2))
            } else if v > 0 {
                Some(Chunk::Empty(v as usize))
            } else {
                None
            }
        })
        .collect_vec()
}

fn rearrange_chunks(fs: &mut Vec<Chunk>) {
    let mut tail = fs.len() - 1;
    'outer: loop {
        // find next file
        while matches!(fs[tail], Chunk::Empty(_)) {
            tail -= 1;
            if tail == 0 {
                break 'outer;
            }
        }
        let size = match fs[tail] {
            Chunk::Empty(_) => 0,
            Chunk::Id(s, _) => s,
        };

        for head in 0..tail {
            let c = fs[head];
            if let Chunk::Empty(empty) = c {
                if empty >= size {
                    if empty == size {
                        fs.swap(head, tail);
                    } else {
                        let extra = Chunk::Empty(empty - size);
                        let swap = Chunk::Empty(size);
                        fs[head] = fs[tail];
                        fs[tail] = swap;
                        fs.insert(head + 1, extra);
                        tail += 1
                    }
                    break;
                }
            }
        }
        tail -= 1;
        if tail == 0 {
            break 'outer;
        }
    }
}

fn rearrange_chunks_short(fs: &mut Vec<Chunk>) {
    let mut tail = fs.len() - 1;
    let mut max_space = usize::MAX;
    'outer: loop {
        // find next file
        while matches!(fs[tail], Chunk::Empty(_)) {
            tail -= 1;
            if tail == 0 {
                break 'outer;
            }
        }
        let size = match fs[tail] {
            Chunk::Empty(_) => 0,
            Chunk::Id(s, _) => s,
        };
        if size > max_space {
            tail -= 1;
            if tail == 0 {
                break 'outer;
            }
            continue;
        }
        let mut swappped = false;
        for head in 0..tail {
            let c = fs[head];
            if let Chunk::Empty(empty) = c {
                if empty >= size {
                    if empty == size {
                        fs.swap(head, tail);
                    } else {
                        let extra = Chunk::Empty(empty - size);
                        let swap = Chunk::Empty(size);
                        fs[head] = fs[tail];
                        fs[tail] = swap;
                        fs.insert(head + 1, extra);
                        tail += 1
                    }
                    swappped = true;
                    break;
                }
            }
        }
        if !swappped {
            max_space = size - 1;
            if max_space == 0 {
                break 'outer;
            }
        }
        tail -= 1;
        if tail == 0 {
            break 'outer;
        }
    }
}

fn checksum_chunks(fs: &[Chunk]) -> usize {
    fs.iter()
        .flat_map(|c| match c {
            Chunk::Empty(size) => vec![0; *size],
            Chunk::Id(size, id) => vec![*id; *size],
        })
        .enumerate()
        .map(|(i, v)| i * v as usize)
        .sum()
}
#[aoc(day9, part2)]
fn solver_part2(input: &Input) -> usize {
    let mut fs = create_chunks(input);
    rearrange_chunks(&mut fs);
    checksum_chunks(&fs)
}

#[aoc(day9, part2, SHORT)]
fn solver_part2_short(input: &Input) -> usize {
    let mut fs = create_chunks(input);
    rearrange_chunks_short(&mut fs);
    checksum_chunks(&fs)
}

#[cfg(test)]
mod tests {
    use crate::day9::solver_part1_inline;

    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "2333133121414131402";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 1928)
    }

    #[test]
    fn part1_inline() {
        assert_eq!(solver_part1_inline(&input_generator(INPUT)), 1928)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 2858)
    }
}
