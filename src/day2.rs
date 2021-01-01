/*
--- Day 2: Password Philosophy ---

Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

// spell-checker:ignore cdefg
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?
Your puzzle answer was 528.

--- Part Two ---
While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

1-3 a: abcde is valid: position 1 contains a and position 3 does not.
1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
How many passwords are valid according to the new interpretation of the policies?

Your puzzle answer was 497.
*/
use crate::parse_error::ParseError;

struct PasswdRule {
    min_occurrence: usize,
    max_occurrence: usize,
    rule_char: char,
    pw: String,
}

#[aoc_generator(day2)]
fn parse_input_pw_list(password_list: &str) -> Vec<PasswdRule> {
    password_list
        .lines()
        .map(|password_check| {
            let mut part = password_check.split(' ');
            let mut occurrence = part.next().ok_or(ParseError::NoneError)?.split('-');
            let min_occurrence: usize = occurrence.next().ok_or(ParseError::NoneError)?.parse()?;
            let max_occurrence: usize = occurrence.next().ok_or(ParseError::NoneError)?.parse()?;
            let rule_char = part
                .next()
                .ok_or(ParseError::NoneError)?
                .chars()
                .next()
                .ok_or(ParseError::NoneError)?;
            let pw = part.next().ok_or(ParseError::NoneError)?;

            Ok(PasswdRule {
                min_occurrence,
                max_occurrence,
                rule_char,
                pw: pw.to_string(),
            })
        })
        .collect::<Result<Vec<PasswdRule>, ParseError>>()
        .expect("Input could not be parsed")
}

#[aoc(day2, part1)]
fn part1(parsed_rules: &[PasswdRule]) -> usize {
    parsed_rules
        .iter()
        .filter(|rule| {
            let occurrences: usize = rule.pw.matches(rule.rule_char).count();
            occurrences >= rule.min_occurrence && occurrences <= rule.max_occurrence
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(parsed_rules: &[PasswdRule]) -> usize {
    parsed_rules
        .iter()
        .filter(|rule| {
            if rule.max_occurrence > rule.pw.chars().count() {
                false
            } else {
                let mut pw_iter = rule.pw.chars();
                let first_char = pw_iter
                    .nth(rule.min_occurrence - 1)
                    .ok_or(ParseError::NoneError);
                let second_char = pw_iter
                    .nth(rule.max_occurrence - rule.min_occurrence - 1)
                    .ok_or(ParseError::NoneError);
                let first_bool = match first_char {
                    Err(_) => false,
                    Ok(first_char) => first_char == rule.rule_char,
                };
                let second_bool = match second_char {
                    Err(_) => false,
                    Ok(second_char) => second_char == rule.rule_char,
                };
                first_bool != second_bool
            }
        })
        .count()
}
