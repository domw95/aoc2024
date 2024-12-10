use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::Lines;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Input = String;

#[derive(Debug, Clone, Copy)]
struct Rule((u32, u32));

impl Rule {
    // fn from_str(str: &str) -> Self {
    //     let t = str.split_once('|').unwrap();
    //     Rule((t.0.parse().unwrap(), t.1.parse().unwrap()))
    // }

    fn from_str_fast(str: &str) -> Self {
        Rule((str[0..2].parse().unwrap(), str[3..5].parse().unwrap()))
    }
}

struct Rules(HashMap<u32, Vec<u32>>);

impl Rules {
    // Returns true if a rule matches the args
    fn matches(&self, before: &u32, after: &u32) -> bool {
        match self.0.get(before) {
            Some(vec) => vec.contains(after),
            None => false,
        }
    }

    fn add_rule(&mut self, rule: Rule) {
        match self.0.get_mut(&rule.0 .0) {
            Some(vec) => vec.push(rule.0 .1),
            None => {
                self.0.insert(rule.0 .0, vec![rule.0 .1]);
            }
        }
    }
}

impl From<&[Rule]> for Rules {
    fn from(rules: &[Rule]) -> Self {
        let mut rules_new = Rules(HashMap::new());
        for &rule in rules {
            rules_new.add_rule(rule);
        }
        rules_new
    }
}

struct RuleMap([Vec<u8>; 100]);

impl RuleMap {
    fn new() -> Self {
        RuleMap(core::array::from_fn(|_| Vec::with_capacity(0)))
    }
    fn add_rule(&mut self, str: &str) {
        let rule: u8 = u8_fast_parse(&str[0..2]);
        let page = u8_fast_parse(&str[3..5]);
        self.0[rule as usize].push(page);
    }

    fn get(&self, key: &u8) -> &Vec<u8> {
        &self.0[*key as usize]
    }

    fn matches(&self, key: &u8, page: &u8) -> bool {
        self.get(key).contains(page)
    }
}

struct RuleMapSet([HashSet<u8>; 100]);

impl RuleMapSet {
    fn new() -> Self {
        RuleMapSet(core::array::from_fn(|_| HashSet::new()))
    }
    fn add_rule(&mut self, str: &str) {
        let rule: u8 = u8_fast_parse(&str[0..2]);
        let page = u8_fast_parse(&str[3..5]);
        self.0[rule as usize].insert(page);
    }

    fn get(&self, key: &u8) -> &HashSet<u8> {
        &self.0[*key as usize]
    }
}

struct RuleMapArray([[bool; 100]; 100]);

impl RuleMapArray {
    fn new() -> Self {
        RuleMapArray([[false; 100]; 100])
    }
    fn add_rule(&mut self, str: &str) {
        let rule: u8 = u8_fast_parse(&str[0..2]);
        let page = u8_fast_parse(&str[3..5]);
        self.0[rule as usize][page as usize] = true;
    }

    fn matches(&self, key: &u8, page: &u8) -> bool {
        self.0[*key as usize][*page as usize]
    }
}

#[derive(Debug, Clone)]
struct Pages(Vec<u32>);

impl Pages {
    fn from_str(str: &str) -> Self {
        Pages(str.split(',').map(|x| x.parse().unwrap()).collect())
    }

    fn is_valid(&self, rules: &[Rule]) -> bool {
        self.0.iter().enumerate().all(|(ind, page)| {
            rules.iter().all(|rule| {
                rule.0 .0 != *page
                    || (self.0[(ind + 1)..].contains(&rule.0 .1)
                        || !self.0[0..ind].contains(&rule.0 .1))
            })
        })
    }

    fn is_valid_hash(&self, rules: &Rules) -> bool {
        self.0
            .iter()
            .enumerate()
            .all(|(ind, page)| match rules.0.get(page) {
                Some(after) => after.iter().all(|after| {
                    self.0[(ind + 1)..].contains(after) || !self.0[0..ind].contains(after)
                }),
                None => true,
            })
    }

    fn is_valid_hash_reduced(&self, rules: &Rules) -> bool {
        self.0
            .iter()
            .enumerate()
            .all(|(ind, page)| match rules.0.get(page) {
                Some(after) => after.iter().all(|after| !self.0[0..ind].contains(after)),
                None => true,
            })
    }

    fn is_valid_fast_hash_reduced(&self, rules: &RuleMap) -> bool {
        self.0.iter().enumerate().all(|(ind, page)| {
            rules
                .get(&(*page as u8))
                .iter()
                .all(|after| !self.0[0..ind].contains(&(*after as u32)))
        })
    }
}

fn parser(input: &str) -> (Vec<Rule>, Vec<Pages>) {
    let mut lines = input.lines();
    let rules = lines
        .take_while_ref(|l| !l.is_empty())
        .map(Rule::from_str_fast)
        .collect();
    lines.next();
    (rules, lines.map(Pages::from_str).collect())
}

fn parser_map(input: &str) -> (Rules, Vec<Pages>) {
    let mut lines = input.lines();
    let mut rules = Rules(HashMap::new());
    for r in lines
        .take_while_ref(|l| !l.is_empty())
        .map(Rule::from_str_fast)
    {
        rules.add_rule(r);
    }
    lines.next();
    (rules, lines.map(Pages::from_str).collect())
}

// fn parser_map_alt(input: &str) -> (Rules, Vec<Pages>) {
//     let (rules, pages) = parser(input);
//     (Rules::from(rules.as_slice()), pages)
// }

fn parser_rule_map(input: &str) -> (RuleMap, Vec<Pages>) {
    let mut rules = RuleMap::new();
    let mut lines = input.lines();
    for l in lines.take_while_ref(|l| !l.is_empty()) {
        rules.add_rule(l);
    }
    lines.next();
    (rules, lines.map(Pages::from_str).collect())
}

fn parser_rule_map_iter(input: &str) -> (RuleMap, Lines) {
    let mut rules = RuleMap::new();
    let mut lines = input.lines();
    for l in lines.take_while_ref(|l| !l.is_empty()) {
        rules.add_rule(l);
    }
    lines.next();
    (rules, lines)
}

fn parser_rule_mapset_iter(input: &str) -> (RuleMapSet, Lines) {
    let mut rules = RuleMapSet::new();
    let mut lines = input.lines();
    for l in lines.take_while_ref(|l| !l.is_empty()) {
        rules.add_rule(l);
    }
    lines.next();
    (rules, lines)
}

fn parser_rule_maparray_iter(input: &str) -> (RuleMapArray, Lines) {
    let mut rules = RuleMapArray::new();
    let mut lines = input.lines();
    for l in lines.take_while_ref(|l| !l.is_empty()) {
        rules.add_rule(l);
    }
    lines.next();
    (rules, lines)
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

#[aoc(day5, part1)]
fn solver_part1(input: &Input) -> u32 {
    let (rules, pages) = parser(input);
    pages
        .iter()
        .filter_map(|pages| {
            if pages.is_valid(&rules) {
                Some(pages.0[pages.0.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part1, HASH)]
fn solver_part1_hash(input: &Input) -> u32 {
    let (rules, pages) = parser_map(input);

    pages
        .iter()
        .filter_map(|pages| {
            if pages.is_valid_hash(&rules) {
                Some(pages.0[pages.0.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part1, HASH_REDUCED)]
fn solver_part1_hash_reduced(input: &Input) -> u32 {
    let (rules, pages) = parser_map(input);

    pages
        .iter()
        .filter_map(|pages| {
            if pages.is_valid_hash_reduced(&rules) {
                Some(pages.0[pages.0.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part1, FAST_HASH_REDUCED)]
fn solver_part1_fast_hash_reduced(input: &Input) -> u32 {
    let (rules, pages) = parser_rule_map(input);

    pages
        .iter()
        .filter_map(|pages| {
            if pages.is_valid_fast_hash_reduced(&rules) {
                Some(pages.0[pages.0.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn u8_fast_parse(str: &str) -> u8 {
    let bytes = str.as_bytes();
    (bytes[0] - 48) * 10 + bytes[1] - 48
}

fn page_line_valid(line: &str, rules: &RuleMap) -> (bool, Vec<u8>) {
    let mut prev = Vec::new();
    (
        line.split(',').map(u8_fast_parse).all(|page| {
            let after = rules.get(&page);
            let res = after.iter().all(|page| !prev.contains(page));
            prev.push(page);
            res
        }),
        prev,
    )
}

fn page_line_invalid(line: &str, rules: &RuleMap) -> (bool, Vec<u8>) {
    let mut prev = Vec::new();

    (
        line.split(',')
            .map(u8_fast_parse)
            .map(|page| {
                let after = rules.get(&page);
                let res = after.iter().all(|page| !prev.contains(page));
                prev.push(page);
                if res {
                    1usize
                } else {
                    0
                }
            })
            .sum::<usize>()
            != prev.len(),
        prev,
    )
}

fn page_line_sorted(line: &str, rules: &RuleMap) -> (bool, Vec<u8>) {
    let pages = line.split(',').map(u8_fast_parse).collect_vec();
    (pages.is_sorted_by(|a, b| !rules.get(b).contains(a)), pages)
}

#[aoc(day5, part1, FAST_HASH_REDUCED_INLINE)]
fn solver_part1_fast_hash_reduced_line(input: &Input) -> u32 {
    let (rules, lines) = parser_rule_map_iter(input);

    lines
        .filter_map(|line| {
            let (ok, vec) = page_line_valid(line, &rules);
            if ok {
                Some((vec[vec.len() / 2]) as u32)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part1, FAST_HASH_INLINE_SORT)]
fn solver_part1_fast_hash_inline_sort(input: &Input) -> u32 {
    let (rules, lines) = parser_rule_map_iter(input);

    lines
        .filter_map(|line| {
            let (ok, vec) = page_line_sorted(line, &rules);
            if ok {
                Some((vec[vec.len() / 2]) as u32)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part1, FAST_HASH_SET_INLINE_SORT)]
fn solver_part1_fast_hash_set_inline_sort(input: &Input) -> u32 {
    let (rules, lines) = parser_rule_mapset_iter(input);

    lines
        .filter_map(|line| {
            let pages = line.split(',').map(u8_fast_parse).collect_vec();

            if pages.is_sorted_by(|a, b| !rules.get(b).contains(a)) {
                Some((pages[pages.len() / 2]) as u32)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part1, FAST_HASH_ARRAY_INLINE_SORT)]
fn solver_part1_fast_hash_array_inline_sort(input: &Input) -> u32 {
    let (rules, lines) = parser_rule_maparray_iter(input);

    lines
        .filter_map(|line| {
            let pages = line.split(',').map(u8_fast_parse).collect_vec();

            if pages.is_sorted_by(|a, b| !rules.matches(b, a)) {
                Some((pages[pages.len() / 2]) as u32)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part2)]
fn solver_part2(input: &Input) -> u32 {
    let (rules, pages) = parser(input);
    pages
        .iter()
        .filter(|pages| !pages.is_valid(&rules))
        .map(|pages| {
            //
            let mut sorted = Vec::new();
            'page: for page in &pages.0 {
                for i in 0..sorted.len() {
                    if rules
                        .iter()
                        .any(|rule| rule.0 .0 == *page && rule.0 .1 == sorted[i])
                    {
                        sorted.insert(i, *page);
                        continue 'page;
                    }
                }
                sorted.push(*page);
            }
            sorted[sorted.len() / 2]
        })
        .sum()
}

#[aoc(day5, part2, HASH)]
fn solver_part2_hash(input: &Input) -> u32 {
    let (rules, pages) = parser_map(input);

    pages
        .iter()
        .filter(|pages| !pages.is_valid_hash(&rules))
        .map(|pages| {
            //
            let mut sorted = Vec::with_capacity(pages.0.len());
            'page: for page in &pages.0 {
                for i in 0..sorted.len() {
                    if rules.matches(page, sorted[i]) {
                        sorted.insert(i, page);
                        continue 'page;
                    }
                }
                sorted.push(page);
            }
            sorted[sorted.len() / 2]
        })
        .sum()
}

fn sort_pages(pages: &[u8], rules: &RuleMap) -> Vec<u8> {
    let mut sorted = Vec::with_capacity(pages.len());
    'outer: for page in pages {
        for i in 0..sorted.len() {
            if rules.matches(page, &sorted[i]) {
                sorted.insert(i, *page);
                continue 'outer;
            }
        }
        sorted.push(*page);
    }
    // 'page: for page in &pages.0 {
    //     for i in 0..sorted.len() {
    //         if rules.matches(page, sorted[i]) {
    //             sorted.insert(i, page);
    //             continue 'page;
    //         }
    //     }
    //     sorted.push(page);
    // }
    sorted
}

#[aoc(day5, part2, FAST_HASH_INLINE)]
fn solver_part2_fast_hash_inline(input: &Input) -> u32 {
    let (rules, lines) = parser_rule_map_iter(input);

    lines
        .filter_map(|line| {
            let (invalid, vec) = page_line_invalid(line, &rules);
            if invalid {
                Some(vec)
            } else {
                None
            }
        })
        .map(|pages| {
            //
            let sorted = sort_pages(&pages, &rules);
            // dbg!(&pages, &sorted);
            sorted[sorted.len() / 2] as u32
        })
        .sum()
}

#[aoc(day5, part2, FAST_HASH_INLINE_SORT)]
fn solver_part2_fast_hash_inline_sort(input: &Input) -> u32 {
    let (rules, lines) = parser_rule_map_iter(input);

    lines
        .filter_map(|line| {
            let (sorted, vec) = page_line_sorted(line, &rules);
            if !sorted {
                Some(vec)
            } else {
                None
            }
        })
        .map(|mut pages| {
            //
            pages.sort_unstable_by(|a, b| {
                if rules.get(a).contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
            pages[pages.len() / 2] as u32
        })
        .sum()
}

#[aoc(day5, part2, FAST_HASH_INLINE_SORT_NTH)]
fn solver_part2_fast_hash_inline_sort_nth(input: &Input) -> u32 {
    let (rules, lines) = parser_rule_map_iter(input);

    lines
        .filter_map(|line| {
            let (sorted, vec) = page_line_sorted(line, &rules);
            if !sorted {
                Some(vec)
            } else {
                None
            }
        })
        .map(|mut pages| {
            //
            let ind = pages.len() / 2;
            *pages
                .select_nth_unstable_by(ind, |a, b| {
                    if rules.get(a).contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                })
                .1 as u32
        })
        .sum()
}

#[aoc(day5, part2, FAST_HASH_ARRAY_INLINE_SORT_NTH)]
fn solver_part2_fast_hash_array_inline_sort_nth(input: &Input) -> u32 {
    let (rules, lines) = parser_rule_maparray_iter(input);

    lines
        .filter_map(|line| {
            let mut pages = line.split(',').map(u8_fast_parse).collect_vec();

            if !pages.is_sorted_by(|a, b| !rules.matches(b, a)) {
                let ind = pages.len() / 2;
                Some(
                    *pages
                        .select_nth_unstable_by(ind, |a, b| {
                            if rules.matches(a, b) {
                                Ordering::Less
                            } else {
                                Ordering::Equal
                            }
                        })
                        .1 as u32,
                )
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day5::solver_part2_fast_hash_inline;

    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 143)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 123)
    }

    #[test]
    fn part2_2() {
        assert_eq!(solver_part2_fast_hash_inline(&input_generator(INPUT)), 123)
    }
}
