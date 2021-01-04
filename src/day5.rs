/*
--- Day 5: Binary Boarding ---

You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.

You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.

Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".

The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.

For example, consider just the first seven characters of FBFBBFFRLR:

    Start by considering the whole range, rows 0 through 127.
    F means to take the lower half, keeping rows 0 through 63.
    B means to take the upper half, keeping rows 32 through 63.
    F means to take the lower half, keeping rows 32 through 47.
    B means to take the upper half, keeping rows 40 through 47.
    B keeps rows 44 through 47.
    F keeps rows 44 through 45.
    The final F keeps the lower of the two, row 44.

The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.

For example, consider just the last 3 characters of FBFBBFFRLR:

    Start by considering the whole range, columns 0 through 7.
    R means to take the upper half, keeping columns 4 through 7.
    L means to take the lower half, keeping columns 4 through 5.
    The final R keeps the upper of the two, column 5.

So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.

Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.

Here are some other boarding passes:

    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBFRLL: row 102, column 4, seat ID 820.

As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?

Your puzzle answer was 938.

The first half of this puzzle is complete! It provides one gold star: *
--- Part Two ---

Ding! The "fasten seat belt" signs have turned on. Time to find your seat.

It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.

Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.

What is the ID of your seat?
696
That's the right answer! You are one gold star closer to saving your vacation.
*/
// use crate::parse_error::ParseError;
use std::cmp;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|str| str.trim().to_string()).collect()
}

fn decode_boarding_pass(boarding_code: &str) -> u32 {
    let (row_code, column_code) = boarding_code.split_at(7);

    let row = bin_convert_direct(row_code); //decode_code_sequence(row_code, 0, 127);
    let column = bin_convert_direct(column_code); // decode_code_sequence(column_code, 0, 7);

    row as u32 * 8 + column as u32
}

// fn decode_code_sequence(code_sequence: &str, lowest: u8, highest: u8) -> u8 {
//     let (lowest, highest) = code_sequence
//         .chars()
//         .fold((lowest, highest), |(lowest, highest), code| {
//             bin_convert_halfen(code, lowest, highest)
//         });
//     // for code in code_sequence.chars() {
//     //     (lowest, highest) = bin_convert_halfen(code, lowest, highest);
//     //     println!("lowest {}, highest {}", lowest, highest)
//     // }

//     assert!(lowest == highest);
//     lowest
// }

// fn bin_convert_halfen(code: char, mut lowest: u8, mut highest: u8) -> (u8, u8) {
//     match code {
//         'F' | 'L' => highest /= 2,
//         'B' | 'R' => lowest = highest / 2,
//         _ => panic!("wrong code letter {}", code),
//     }
//     (lowest, highest)
// }

fn bin_convert_direct(code_sequence: &str) -> u8 {
    let bin: String = code_sequence
        .chars()
        .map(|code| match code {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => panic!("wrong code letter {}", code),
        })
        .collect();
    u8::from_str_radix(&bin, 2).unwrap()
}

#[aoc(day5, part1)]
fn part1(boarding_passes: &[String]) -> u32 {
    boarding_passes.iter().fold(0, |highest, boarding_code| {
        cmp::max(highest, decode_boarding_pass(boarding_code))
    })
}

#[aoc(day5, part2, iterative)]
fn part2(boarding_passes: &[String]) -> u32 {
    let mut seat_numbers = boarding_passes
        .iter()
        .map(|boarding_code| decode_boarding_pass(boarding_code))
        .collect::<Vec<u32>>();

    seat_numbers.sort_unstable();

    let mut seats_iter = seat_numbers.iter();
    let mut previous = seats_iter.next().unwrap();

    let mut found_seat: u32 = 0;

    for seat in seats_iter {
        match seat - previous {
            2 => {
                found_seat = seat - 1;
                break;
            }
            _ => previous = seat,
        }
    }

    found_seat
}

#[aoc(day5, part2, functional)]
fn part2_functional(boarding_passes: &[String]) -> u32 {
    let mut seat_numbers = boarding_passes
        .iter()
        .map(|boarding_code| decode_boarding_pass(boarding_code))
        .collect::<Vec<u32>>();

    seat_numbers.sort_unstable();

    seat_numbers
        .iter()
        .fold(0, |previous: u32, seat| match seat - previous {
            2 => seat - 1,
            _ => *seat,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input_part1() -> &'static str {
        "FBFBBFFRLR
         BFFFBBFRRR
         FFFBBBFRRR
         BBFFBBFLRL
         BBFFBBFRLL"
    }
    fn parse_test_input() -> Vec<String> {
        parse_input(sample_input_part1())
    }
    #[test]
    fn test_row_and_seat_resolution() {
        let boarding_code = "FBFBBFFRLR";
        assert_eq!(357, decode_boarding_pass(boarding_code));
        let boarding_code = "BFFFBBFRRR";
        assert_eq!(567, decode_boarding_pass(boarding_code));
        let boarding_code = "FFFBBBFRRR";
        assert_eq!(119, decode_boarding_pass(boarding_code));
        let boarding_code = "BBFFBBFLRL";
        assert_eq!(818, decode_boarding_pass(boarding_code));
        let boarding_code = "BBFFBBFRLL";
        assert_eq!(820, decode_boarding_pass(boarding_code));
    }
    #[test]
    fn test_max_seat_id() {
        let boarding_code = "BBBBBBBRRR";
        assert_eq!(1023, decode_boarding_pass(boarding_code));
    }

    #[test]
    fn bin_convert_direct_test() {
        assert_eq!(5, bin_convert_direct("RLR"));
        assert_eq!(44, bin_convert_direct("FBFBBFF"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(820, part1(&parse_test_input()))
    }

    #[test]
    fn test_part2() {
        assert_eq!(819, part2(&parse_test_input()))
    }
    #[test]
    fn test_part2_functional() {
        assert_eq!(819, part2_functional(&parse_test_input()))
    }
}
