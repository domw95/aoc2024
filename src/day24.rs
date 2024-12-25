use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use fxhash::FxHashMap;
use fxhash::FxHashSet;
use itertools::Itertools;

type Input = String;

#[derive(Debug, Clone, Copy)]
struct Gate<'a> {
    inputs: (GateInput<'a>, GateInput<'a>),
    op: GateOP,
    value: Option<bool>,
}

#[derive(Debug, Clone, Copy)]
enum GateInput<'a> {
    Gate(&'a str),
    Value(bool),
}
impl Gate<'_> {
    fn new<'a>(op: GateOP, a: GateInput<'a>, b: GateInput<'a>) -> Gate<'a> {
        Gate {
            inputs: (a, b),
            op,
            value: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Gate2<'a> {
    inputs: (&'a str, &'a str),
    op: GateOP,
}

impl Gate2<'_> {
    fn new<'a>(op: GateOP, a: &'a str, b: &'a str) -> Gate2<'a> {
        Gate2 { inputs: (a, b), op }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum GateOP {
    Xor,
    And,
    Or,
}

fn parse(input: &str) -> (FxHashMap<&str, bool>, FxHashMap<&str, Gate>, u32) {
    let mut lines = input.lines();
    let signals: FxHashMap<_, _> = lines
        .take_while_ref(|line| !line.is_empty())
        .map(|line| {
            let (k, v) = line.split_once(": ").unwrap();
            (k, v.as_bytes()[0] == b'1')
        })
        .collect();
    lines.next();
    let mut gates = FxHashMap::default();
    let mut size = 0;
    for line in lines {
        let mut line = line.split_ascii_whitespace();
        let in1 = line.next().unwrap();
        let op = match line.next().unwrap() {
            "XOR" => GateOP::Xor,
            "OR" => GateOP::Or,
            _ => GateOP::And,
        };
        let in2 = line.next().unwrap();
        line.next();
        let out = line.next().unwrap();

        let in1 = if let Some(sig) = signals.get(in1) {
            GateInput::Value(*sig)
        } else {
            GateInput::Gate(in1)
        };

        let in2 = if let Some(sig) = signals.get(in2) {
            GateInput::Value(*sig)
        } else {
            GateInput::Gate(in2)
        };
        if &out[0..1] == "z" {
            let new_size = out[1..].parse().unwrap();
            size = size.max(new_size);
        }
        gates.insert(out, Gate::new(op, in1, in2));
    }
    (signals, gates, size)
}

fn parse2(input: &str) -> (FxHashMap<&str, Gate2>, u32) {
    let mut lines = input.lines();
    lines.take_while_ref(|line| !line.is_empty()).count();

    lines.next();
    let mut gates = FxHashMap::default();
    let mut size = 0;
    for line in lines {
        let mut line = line.split_ascii_whitespace();
        let in1 = line.next().unwrap();
        let op = match line.next().unwrap() {
            "XOR" => GateOP::Xor,
            "OR" => GateOP::Or,
            _ => GateOP::And,
        };
        let in2 = line.next().unwrap();
        let (in1, in2) = {
            if &in1[0..1] == "x" {
                (in1, in2)
            } else {
                (in2, in1)
            }
        };
        line.next();
        let out = line.next().unwrap();
        if &out[0..1] == "z" {
            let new_size = out[1..].parse().unwrap();
            size = size.max(new_size);
        }
        gates.insert(out, Gate2::new(op, in1, in2));
    }
    (gates, size)
}

#[aoc_generator(day24)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn get_input_values(key: &str, gates: &mut FxHashMap<&str, Gate>) -> (bool, bool) {
    let in1 = gates.get(key).unwrap().inputs.0;
    let in2 = gates.get(key).unwrap().inputs.1;
    (
        match in1 {
            GateInput::Gate(key) => get_gate_value(key, gates),
            GateInput::Value(b) => b,
        },
        match in2 {
            GateInput::Gate(key) => get_gate_value(key, gates),
            GateInput::Value(b) => b,
        },
    )
}

fn get_gate_value(key: &str, gates: &mut FxHashMap<&str, Gate>) -> bool {
    let value = gates.get(key).unwrap().value;
    if let Some(b) = value {
        b
    } else {
        let (in1, in2) = get_input_values(key, gates);
        match gates.get(key).unwrap().op {
            GateOP::And => in1 && in2,
            GateOP::Or => in1 | in2,
            GateOP::Xor => in1 ^ in2,
        }
    }
}

#[aoc(day24, part1)]
fn solver_part1(input: &Input) -> u64 {
    let (_, mut gates, max) = parse(input);
    let mut out = 0u64;
    for z in 0..=max {
        let key = format!("z{z:02}");
        if get_gate_value(&key, &mut gates) {
            out += 1 << z;
        }
    }
    out
}

#[aoc(day24, part2)]
fn solver_part2(input: &Input) -> String {
    let (gates, _) = parse2(input);

    let zgates = gates
        .iter()
        .filter(|(out, gate)| {
            (&out[0..1] == "z" && gate.op != GateOP::Xor)
                & !(**out == "z45" && gate.op == GateOP::Or)
        })
        .collect_vec();

    let xorgates = gates
        .iter()
        .filter(|(out, gate)| {
            gate.op == GateOP::Xor
                && ((&gate.inputs.0[0..1] == "x") == (&out[0..1] == "z")
                    && &gate.inputs.0[0..] != "x00")
        })
        .collect_vec();

    let mut swaps = zgates
        .into_iter()
        .flat_map(|z| {
            let i = &z.0[1..];
            for xor in &xorgates {
                let left = gates.get(xor.1.inputs.0).unwrap();
                let right = gates.get(xor.1.inputs.1).unwrap();
                if left.inputs.0 == format!("x{i:02}") || right.inputs.0 == format!("x{i:02}") {
                    return [z.0, xor.0];
                }
            }
            panic!()
        })
        .collect_vec();

    let invalid = gates
        .iter()
        .filter_map(|(_, gate)| {
            if gate.op == GateOP::Or {
                let in1 = gates.get(gate.inputs.0).unwrap();
                let in2 = gates.get(gate.inputs.1).unwrap();
                if in1.op != GateOP::And {
                    Some((gate.inputs.0, in1))
                } else if in2.op != GateOP::And {
                    Some((gate.inputs.1, in2))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter(|g| !swaps.contains(&&g.0))
        .collect_vec();

    let good_or: FxHashSet<_> = gates
        .iter()
        .filter_map(|(_, gate)| {
            if gate.op == GateOP::Or {
                if gates.get(gate.inputs.0).unwrap().op == GateOP::And {
                    if gates.get(gate.inputs.1).unwrap().op == GateOP::And {
                        Some(vec![gate.inputs.0, gate.inputs.1])
                    } else {
                        Some(vec![gate.inputs.0])
                    }
                } else if gates.get(gate.inputs.1).unwrap().op == GateOP::And {
                    Some(vec![gate.inputs.1])
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .collect();

    let andgates = gates
        .iter()
        .filter(|(out, gate)| {
            gate.op == GateOP::And && !good_or.contains(**out) && gate.inputs.0 != "x00"
        })
        .collect_vec();

    let wrong = andgates
        .iter()
        .chain(xorgates.iter())
        .filter(|(out, _)| !swaps.contains(out))
        .collect_vec();
    swaps.push(wrong[0].0);
    swaps.push(&invalid[0].0);
    // dbg!(gates.values().filter(|g| g.op == GateOP::Xor).count());
    // dbg!(gates.values().filter(|g| g.op == GateOP::And).count());
    // dbg!(gates.values().filter(|g| g.op == GateOP::Or).count());
    swaps.sort_unstable();
    swaps.into_iter().join(",")
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;

    static INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 2024)
    }
}
