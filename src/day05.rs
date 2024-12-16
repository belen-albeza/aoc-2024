use aoc_runner_derive::aoc;
use std::cmp::Ordering;

type Rule = (u32, u32);
type Update = Vec<u32>;

fn parse_part1(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let rules = sections[0]
        .lines()
        .map(|line| {
            let pages = line
                .split("|")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>();
            (pages[0], pages[1])
        })
        .collect();

    let updates = sections[1]
        .lines()
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

struct Ruleset {
    rules: Vec<Rule>,
}

impl Ruleset {
    fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    fn valid_update(&self, update: &Update) -> bool {
        let mut sorted = update.clone();
        sorted.sort_by(self.sort_fn());
        sorted == *update
    }

    fn sort_update(&self, update: &Update) -> Option<Update> {
        let mut res = update.clone();
        res.sort_by(self.sort_fn());
        Some(res)
    }

    fn sort_fn<'a>(&self) -> impl Fn(&u32, &u32) -> Ordering + use<'_> {
        |a, b| {
            let binding = self.rules_for(*a);
            let rule = binding.iter().find(|(_, y)| y == b);
            if a == b {
                Ordering::Equal
            } else if rule.is_some() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }

    fn rules_for(&self, page: u32) -> Vec<Rule> {
        self.rules
            .iter()
            .filter(|(x, _)| *x == page)
            .copied()
            .collect()
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    let (rules, updates) = parse_part1(input);
    let ruleset = Ruleset::new(rules);

    updates
        .iter()
        .filter_map(|update| {
            if ruleset.valid_update(update) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u32 {
    let (rules, updates) = parse_part1(input);
    let ruleset = Ruleset::new(rules);

    updates
        .into_iter()
        .filter(|update| !ruleset.valid_update(update))
        .map(|update| ruleset.sort_update(&update).unwrap())
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"47|53
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
    fn part1_example() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 123);
    }

    #[test]
    fn rules_for() {
        let (rules, _) = parse_part1(INPUT);
        let ruleset = Ruleset::new(rules);

        assert_eq!(
            ruleset.rules_for(97),
            vec![(97, 13), (97, 61), (97, 47), (97, 29), (97, 53), (97, 75)]
        );
    }

    #[test]
    fn sort_update() {
        let (rules, _) = parse_part1(INPUT);
        let ruleset = Ruleset::new(rules);

        assert_eq!(
            ruleset.sort_update(&vec![75, 97, 47, 61, 53]),
            Some(vec![97, 75, 47, 61, 53])
        );
    }
}
