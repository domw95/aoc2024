use std::collections::HashMap;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Input = (Vec<Rule>, Vec<Pages>);

#[derive(Debug, Clone, Copy)]
struct Rule((u32, u32));

impl Rule {
    fn from_str(str: &str) -> Self {
        let t = str.split_once('|').unwrap();
        Rule((t.0.parse().unwrap(), t.1.parse().unwrap()))
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
}

impl From<&[Rule]> for Rules {
    fn from(rules: &[Rule]) -> Self {
        let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
        for rule in rules {
            match map.get_mut(&rule.0 .0) {
                Some(vec) => vec.push(rule.0 .1),
                None => {
                    map.insert(rule.0 .0, vec![rule.0 .1]);
                }
            }
        }
        Rules(map)
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
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let rules = lines
        .take_while_ref(|l| !l.is_empty())
        .map(Rule::from_str)
        .collect();
    lines.next();
    (rules, lines.map(Pages::from_str).collect())
}

#[aoc(day5, part1)]
fn solver_part1((rules, pages): &Input) -> u32 {
    pages
        .iter()
        .filter_map(|pages| {
            if pages.is_valid(rules) {
                Some(pages.0[pages.0.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part1, HASH)]
fn solver_part1_hash((rules, pages): &Input) -> u32 {
    let rules = Rules::from(rules.as_slice());
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
#[aoc(day5, part2)]
fn solver_part2((rules, pages): &Input) -> u32 {
    pages
        .iter()
        .filter(|pages| !pages.is_valid(rules))
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
fn solver_part2_hash((rules, pages): &Input) -> u32 {
    let rules = Rules::from(rules.as_slice());
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

#[cfg(test)]
mod tests {
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
}
