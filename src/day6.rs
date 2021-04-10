/*
--- Day 6: Custom Customs ---

As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.

The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.

However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each of the people in their group, you write down the questions for which they answer "yes", one per line. For example:

abcx
abcy
abcz

In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the same question don't count extra; each question counts at most once.)

Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    The first group contains one person who answered "yes" to 3 questions: a, b, and c.
    The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
    The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
    The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
    The last group contains one person who answered "yes" to only 1 question, b.

In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.

For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?

Your puzzle answer was 6351.

The first half of this puzzle is complete! It provides one gold star: *
--- Part Two ---

As you finish the last group's customs declaration, you notice that you misread one word in the instructions:

You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!

Using the same example as above:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
    In the second group, there is no question to which everyone answered "yes".
    In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
    In the fourth group, everyone answered yes to only 1 question, a.
    In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.

In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?

3143
*/
use std::collections::{HashMap, HashSet};

// use crate::parse_error::ParseError;
use regex::Regex;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Vec<String>> {
    let re = Regex::new(r"\n\s*\n").unwrap();
    let group = re.split(input);

    group
        .map(|answers| answers.lines().map(|str| str.trim().to_string()).collect())
        .collect()
}

#[aoc(day6, part1)]
fn part1(grouped_answers: &[Vec<String>]) -> u32 {
    grouped_answers.iter().fold(0, |sum, answers| {
        sum + count_yes_answers_from_anyone(answers)
    })
}

fn count_yes_answers_from_anyone(answers: &[String]) -> u32 {
    let mut unique_answers: HashSet<char> = HashSet::new();
    answers.iter().flat_map(|s| s.chars()).for_each(|c| {
        unique_answers.insert(c);
    });
    unique_answers.len() as u32
}

#[aoc(day6, part2)]
fn part2(grouped_answers: &[Vec<String>]) -> u32 {
    grouped_answers.iter().fold(0, |sum, answers| {
        sum + count_yes_answers_from_everyone(answers)
    })
}

fn count_yes_answers_from_everyone(answers: &[String]) -> u32 {
    let answerers = answers.len() as u8;
    if answerers == 1 {
        return answers.first().expect("got an empty answer!").len() as u32;
    }
    let mut all_answers: HashMap<char, u8> = HashMap::new();
    answers.iter().flat_map(|s| s.chars()).for_each(|c| {
        all_answers.insert(c, {
            match all_answers.get(&c) {
                None => 1,
                Some(n) => n + 1,
            }
        });
    });

    all_answers
        .keys()
        .filter(|key| *all_answers.get(*key).unwrap() == answerers)
        .count() as u32
}
// #[aoc(day6, part2)]
// fn part2(boarding_passes: &[String]) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b";

    fn parse_test_input() -> Vec<Vec<String>> {
        parse_input(SAMPLE_INPUT)
    }

    #[test]
    fn test_count_yes_answers_from_anyone() {
        assert_eq!(3, count_yes_answers_from_anyone(&[String::from("abc")]));
        assert_eq!(
            3,
            count_yes_answers_from_anyone(&[
                String::from("a"),
                String::from("b"),
                String::from("c")
            ])
        );
        assert_eq!(
            3,
            count_yes_answers_from_anyone(&[String::from("ab"), String::from("ac")])
        );
        assert_eq!(
            1,
            count_yes_answers_from_anyone(&[
                String::from("a"),
                String::from("a"),
                String::from("a"),
                String::from("a")
            ])
        );
        assert_eq!(1, count_yes_answers_from_anyone(&[String::from("b")]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(11, part1(&parse_test_input()))
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(&parse_test_input()))
    }
}
