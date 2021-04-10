/*
--- Day 7: Handy Haversacks ---

You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!

For example, consider the following rules:

light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.

These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

    A bright white bag, which can hold your shiny gold bag directly.
    A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.

So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)

Your puzzle answer was 378.

The first half of this puzzle is complete! It provides one gold star: *
--- Part Two ---

It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!

Consider again your shiny gold bag and the rules from the above example:

    faded blue bags contain 0 other bags.
    dotted black bags contain 0 other bags.
    vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
    dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.

So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!

Here's another example:

shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.

In this example, a single shiny gold bag must contain 126 other bags.

How many individual bags are required inside your single shiny gold bag?
27526
That's the right answer! You are one gold star closer to saving your vacation.
*/

use std::collections::HashMap;

use regex::Regex;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> HashMap<String, Option<Vec<(u8, String)>>> {
    let re = Regex::new(r"(.*) bags contain (.*)+").unwrap();
    let mut result = HashMap::new();

    input.lines().for_each(|str| {
        let caps = re.captures(str).expect("parse error!");
        // mirrored magenta bags contain 3 mirrored gray bags, 2 plaid beige bags, 4 dull brown bags, 3 pale plum bags.
        let key = String::from(&caps[1]);

        let value = match &caps[2] {
            s if s.starts_with("no other") => None,
            s => Some(
                s.split(", ")
                    .map(|count_bag| {
                        let count = count_bag
                            .split(' ')
                            .next()
                            .expect("parse error!")
                            .parse::<u8>()
                            .expect("parse error!");
                        let mut not_stripped_bag = count_bag
                            .strip_prefix(format!("{} ", count).as_str())
                            .expect("parse error!");
                        not_stripped_bag = not_stripped_bag
                            .strip_suffix('.')
                            .unwrap_or(not_stripped_bag);
                        let bag = String::from(
                            not_stripped_bag.strip_suffix(" bag").unwrap_or_else(|| {
                                not_stripped_bag
                                    .strip_suffix(" bags")
                                    .expect("parse error!")
                            }),
                        );
                        (count, bag)
                    })
                    .collect(),
            ),
        };
        result.insert(key, value);
    });
    result
}

#[aoc(day7, part1)]
fn part1(bags: &HashMap<String, Option<Vec<(u8, String)>>>) -> u32 {
    bags.keys()
        .filter(|bag| contains_shiny_gold_bag(*bag, bags, &mut HashMap::new()))
        .count() as u32
        - 1
}

fn contains_shiny_gold_bag(
    bag: &str,
    all_bags: &HashMap<String, Option<Vec<(u8, String)>>>,
    bag_contains_golden: &mut HashMap<String, bool>,
) -> bool {
    let contains = match bag_contains_golden.get(bag) {
        Some(contains) => *contains,
        None => {
            if bag == "shiny gold" {
                true
            } else {
                match all_bags.get(bag) {
                    None => panic!("unknown bag {}", bag),
                    Some(inner_bags) => match inner_bags {
                        None => false,
                        Some(inner_bags) => inner_bags.iter().any(|(_, bag)| {
                            contains_shiny_gold_bag(bag, all_bags, bag_contains_golden)
                        }),
                    },
                }
            }
        }
    };
    contains
}

#[aoc(day7, part2)]
fn part2(bags: &HashMap<String, Option<Vec<(u8, String)>>>) -> u32 {
    number_of_contained_bags("shiny gold", bags, &mut HashMap::new())
}

fn number_of_contained_bags(
    bag: &str,
    all_bags: &HashMap<String, Option<Vec<(u8, String)>>>,
    contained_bags: &mut HashMap<String, u32>,
) -> u32 {
    let bags_count = match contained_bags.get(bag) {
        Some(contained_bags) => *contained_bags,
        None => match all_bags.get(bag) {
            None => panic!("unknown bag {}", bag),
            Some(inner_bags) => match inner_bags {
                None => 0,
                Some(inner_bags) => inner_bags.iter().fold(0, |sum, (count, bag)| {
                    let count_inner_bags = number_of_contained_bags(bag, all_bags, contained_bags);
                    let count = *count as u32;
                    sum + count + (count * count_inner_bags)
                }),
            },
        },
    };
    contained_bags.insert(String::from(bag), bags_count);
    bags_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_PART1: &str =
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const SAMPLE_INPUT_PART2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    fn parse_test_input() -> HashMap<String, Option<Vec<(u8, String)>>> {
        parse_input(SAMPLE_INPUT_PART1)
    }

    fn parse_test_input_part2() -> HashMap<String, Option<Vec<(u8, String)>>> {
        parse_input(SAMPLE_INPUT_PART2)
    }

    #[test]
    fn test_part1() {
        assert_eq!(4, part1(&parse_test_input()))
    }

    #[test]
    fn test_part2() {
        assert_eq!(32, part2(&parse_test_input()));
    }

    #[test]
    fn test_number_of_contained_bags_dark_violet() {
        assert_eq!(
            0,
            number_of_contained_bags(
                "dark violet",
                &parse_test_input_part2(),
                &mut HashMap::new(),
            )
        );
    }

    #[test]
    fn test_number_of_contained_bags_dark_blue() {
        assert_eq!(
            2,
            number_of_contained_bags("dark blue", &parse_test_input_part2(), &mut HashMap::new())
        );
    }

    #[test]
    fn test_number_of_contained_bags_dark_green() {
        assert_eq!(
            6,
            number_of_contained_bags("dark green", &parse_test_input_part2(), &mut HashMap::new())
        );
    }

    #[test]
    fn test_number_of_contained_bags_dark_yellow() {
        assert_eq!(
            14,
            number_of_contained_bags(
                "dark yellow",
                &parse_test_input_part2(),
                &mut HashMap::new(),
            )
        );
    }
}
