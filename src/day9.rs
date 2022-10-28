/*
--- Day 9: Encoding Error ---

With your neighbor happily enjoying their video game, you turn your attention to an open data port on the little screen in the seat in front of you.

Though the port is non-standard, you manage to connect it to your computer through the clever use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle input).

The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which, conveniently for you, is an old cypher with an important weakness.

XMAS starts by transmitting a preamble of 25 numbers. After that, each number you receive should be the sum of any two of the 25 immediately previous numbers. The two numbers will have different values, and there might be more than one such pair.

For example, suppose your preamble consists of the numbers 1 through 25 in a random order. To be valid, the next number must be the sum of two of those numbers:

26 would be a valid next number, as it could be 1 plus 25 (or many other pairs, like 2 and 24).
49 would be a valid next number, as it is the sum of 24 and 25.
100 would not be valid; no two of the previous 25 numbers sum to 100.
50 would also not be valid; although 25 appears in the previous 25 numbers, the two numbers in the pair must be different.
Suppose the 26th number is 45, and the first number (no longer an option, as it is more than 25 numbers ago) was 20. Now, for the next number to be valid, there needs to be some pair of numbers among 1-19, 21-25, or 45 that add up to it:

26 would still be a valid next number, as 1 and 25 are still within the previous 25 numbers.
65 would not be valid, as no two of the available numbers sum to it.
64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.
Here is a larger example which only considers the previous 5 numbers (and has a preamble of length 5):

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
In this example, after the 5-number preamble, almost every number is the sum of two of the previous 5 numbers;
the only number that does not follow this rule is 127.

The first step of attacking the weakness in the XMAS data is to find the first number in the list (after the preamble)
which is not the sum of two of the 25 numbers before it. What is the first number that does not have this property?
20874512

--- Part Two ---

The final step in breaking the XMAS encryption relies on the invalid number you just found: you must find a contiguous
set of at least two numbers in your list which sum to the invalid number from step 1.

Again consider the above example:

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
In this list, adding up all of the numbers from 15 through 40 produces the invalid number from step 1, 127. (Of course, the contiguous set of numbers in your actual list
might be much longer.)

To find the encryption weakness, add together the smallest and largest number in this contiguous range; in this example, these are 15 and 47, producing 62.

What is the encryption weakness in your XMAS-encrypted list of numbers?
3012420
*/

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|e| {
            e.parse()
                .unwrap_or_else(|_| panic!("could not parse {}", e))
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[isize]) -> isize {
    find_weakness(input, 26)
}

fn find_weakness(input: &[isize], prelude_length: usize) -> isize {
    let mut prelude_offset = 0;
    let code = &input[prelude_length..];
    let mut found_sum = false;
    for cipher in code {
        let prelude = &input[prelude_offset..prelude_length + prelude_offset];
        for addend_1 in prelude {
            let addend_2 = cipher - addend_1;
            if addend_2 == *addend_1 {
                continue;
            } else if prelude.contains(&addend_2) {
                found_sum = true;
                break;
            }
        }
        if found_sum {
            //reset for next cipher
            found_sum = false;
        } else {
            //found the weakness!
            return *cipher;
        }
        //shift the prelude
        prelude_offset += 1;
    }
    panic!("no weakness found!");
}

#[aoc(day9, part2)]
fn part2(input: &[isize]) -> isize {
    break_code(input, 26)
}
fn break_code(input: &[isize], prelude_length: usize) -> isize {
    let weakness_sum = find_weakness(input, prelude_length);
    let mut sum;
    let mut seq_last_index;
    for (index, i) in input.iter().enumerate() {
        seq_last_index = index;
        sum = *i;
        while sum < weakness_sum {
            seq_last_index += 1;
            sum += input[seq_last_index]
        }
        if sum == weakness_sum {
            //found sequence, now adding min and max
            let sequence = &input[index..seq_last_index + 1];
            return sum_min_max(sequence);
        }
    }
    panic!("no breach found!");
}

fn sum_min_max(sequence: &[isize]) -> isize {
    let mut min = sequence[0];
    let mut max = sequence[1];
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }
    for i in &sequence[2..] {
        if *i < min {
            min = *i;
        } else if *i > max {
            max = *i;
        }
    }
    min + max
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    fn parse_test_input() -> Vec<isize> {
        parse_input(SAMPLE_INPUT)
    }

    #[test]
    fn test_part1() {
        assert_eq!(127, find_weakness(&parse_test_input(), 5))
    }

    #[test]
    fn test_part2() {
        assert_eq!(62, break_code(&parse_test_input(), 5))
    }
}
