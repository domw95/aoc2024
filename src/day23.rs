use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Input = String;

#[aoc_generator(day23)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

#[derive(Debug, Default, Clone)]
struct Node {
    connections: Vec<usize>,
    t: bool,
}

impl Node {
    fn new(t: bool) -> Self {
        Self {
            t,
            ..Default::default()
        }
    }
    fn add_connection(&mut self, c: usize) {
        self.connections.push(c);
    }
}

fn hash(s: &[u8; 2]) -> (usize, bool) {
    let ind = (s[0] as usize - 97) * 26;
    (ind + (s[1] - 97) as usize, s[0] == b't')
}

#[aoc(day23, part1)]
fn solver_part1(input: &Input) -> usize {
    let mut nodes: [Option<Node>; 768] = [const { None }; 768];
    for line in input.lines() {
        let bytes = line.as_bytes();
        let (ind1, t1) = hash(&bytes[0..2].try_into().unwrap());
        let (ind2, t2) = hash(&bytes[3..5].try_into().unwrap());
        if let Some(node1) = &mut nodes[ind1] {
            node1.add_connection(ind2);
        } else {
            let mut node1 = Node::new(t1);
            node1.add_connection(ind2);
            nodes[ind1] = Some(node1);
        }

        if let Some(node2) = &mut nodes[ind2] {
            node2.add_connection(ind1);
        } else {
            let mut node2 = Node::new(t2);
            node2.add_connection(ind1);
            nodes[ind2] = Some(node2);
        }
    }
    nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| n.as_ref().map(|n| (i, n)))
        .map(|(ind, node)| {
            let mut count = 0;

            for ind2 in &node.connections {
                let node2 = nodes[*ind2].as_ref().unwrap();

                for ind3 in &node2.connections {
                    let node3 = nodes[*ind3].as_ref().unwrap();
                    if (node.t | node2.t | node3.t) && node3.connections.contains(&ind) {
                        count += 1;
                    }
                }
            }
            count
        })
        .sum::<usize>()
        / 6
}

fn ordered_hashes(bytes: &[u8]) -> ((usize, bool), (usize, bool)) {
    let a = hash(&bytes[0..2].try_into().unwrap());
    let b = hash(&bytes[3..5].try_into().unwrap());
    if a.0 > b.0 {
        (b, a)
    } else {
        (a, b)
    }
}

#[aoc(day23, part1, FASTER)]
fn solver_part1_faster(input: &Input) -> usize {
    let mut nodes: [Option<Node>; 768] = [const { None }; 768];
    for line in input.lines() {
        let bytes = line.as_bytes();
        let ((ind1, t1), (ind2, t2)) = ordered_hashes(bytes);

        if let Some(node1) = &mut nodes[ind1] {
            node1.add_connection(ind2);
        } else {
            let mut node1 = Node::new(t1);
            node1.add_connection(ind2);
            nodes[ind1] = Some(node1);
        }

        if nodes[ind2].is_none() {
            let node2 = Node::new(t2);
            nodes[ind2] = Some(node2);
        }
    }
    nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| n.as_ref().map(|n| (i, n)))
        .map(|(_, node)| {
            let mut count = 0;

            for ind2 in node.connections.iter() {
                let node2 = nodes[*ind2].as_ref().unwrap();

                for ind3 in &node2.connections {
                    let node3 = nodes[*ind3].as_ref().unwrap();
                    if (node.t | node2.t | node3.t) && node.connections.contains(ind3) {
                        count += 1
                    }
                }
            }
            count
        })
        .sum::<usize>()
}

#[aoc(day23, part1, SORTED)]
fn solver_part1_sorted(input: &Input) -> usize {
    let mut nodes: [Option<Node>; 768] = [const { None }; 768];
    for line in input.lines() {
        let bytes = line.as_bytes();
        let ((ind1, t1), (ind2, t2)) = ordered_hashes(bytes);

        if let Some(node1) = &mut nodes[ind1] {
            node1.add_connection(ind2);
        } else {
            let mut node1 = Node::new(t1);
            node1.add_connection(ind2);
            nodes[ind1] = Some(node1);
        }

        if nodes[ind2].is_none() {
            let node2 = Node::new(t2);
            nodes[ind2] = Some(node2);
        }
    }

    // sort the nodes for ordered search space
    for node in nodes.iter_mut().flatten() {
        node.connections.sort_unstable();
    }
    nodes
        .iter()
        .flatten()
        .map(|node| {
            let mut count = 0;

            for (i, ind2) in node.connections.iter().enumerate() {
                let node2 = nodes[*ind2].as_ref().unwrap();

                for ind3 in &node2.connections {
                    let node3 = nodes[*ind3].as_ref().unwrap();
                    if (node.t | node2.t | node3.t) && node.connections[i..].contains(ind3) {
                        count += 1
                    }
                }
            }
            count
        })
        .sum::<usize>()
}

fn hash2id(ind: usize) -> String {
    let mut string = String::new();
    let c1 = ind / 26;
    let c2 = ind % 26;
    string.push((c1 as u8 + 97) as char);
    string.push((c2 as u8 + 97) as char);
    string
}

#[aoc(day23, part2)]
fn solver_part2(input: &Input) -> String {
    let mut nodes: [Option<Node>; 768] = [const { None }; 768];
    for line in input.lines() {
        let bytes = line.as_bytes();
        let ((ind1, t1), (ind2, t2)) = ordered_hashes(bytes);

        if let Some(node1) = &mut nodes[ind1] {
            node1.add_connection(ind2);
        } else {
            let mut node1 = Node::new(t1);
            node1.add_connection(ind2);
            nodes[ind1] = Some(node1);
        }

        if nodes[ind2].is_none() {
            let node2 = Node::new(t2);
            nodes[ind2] = Some(node2);
        }
    }

    // sort the nodes for ordered search space
    for node in nodes.iter_mut().flatten() {
        node.connections.sort_unstable();
    }
    let mut best = Vec::new();
    for ind in 0..nodes.len() {
        if nodes[ind].is_none() {
            continue;
        }
        let mut state = Vec::new();
        state.push((ind, 0usize));
        'outer: loop {
            // println!("{state:?}");
            let (ind, i) = state.last().unwrap().to_owned();
            let node = nodes[ind].as_ref().unwrap();

            // go through remaining unchecked connections
            for (j, ind) in node.connections[i..].iter().enumerate() {
                if state.len() == 1
                    || state[0..(state.len() - 1)].iter().all(|(ind_2, i_2)| {
                        nodes[*ind_2].as_ref().unwrap().connections[*i_2..].contains(ind)
                    })
                {
                    // remember position for next time at node
                    state.last_mut().unwrap().1 = i + j;
                    // Insert this node and continue
                    state.push((*ind, 0));
                    continue 'outer;
                }
            }
            if state.len() > best.len() {
                best = state.clone()
            }
            state.pop();
            if state.is_empty() {
                break;
            } else {
                state.last_mut().unwrap().1 += 1;
            }
        }
    }
    let mut best: Vec<_> = best.into_iter().map(|(ind, _)| hash2id(ind)).collect();
    // dbg!(&best);
    best.sort();
    // dbg!(&best);
    best.join(",").to_string()
}

#[cfg(test)]
mod tests {
    use crate::day23::solver_part1_faster;

    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 7)
    }

    #[test]
    fn part1_2() {
        assert_eq!(solver_part1_faster(&input_generator(INPUT)), 7)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), "co,de,ka,ta")
    }
}
