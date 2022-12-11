//use advent_of_code_2020::aoc2020::*;
mod aoc2020;
mod utils;
use aoc2020::*;

aoc_main::main! {
    year 2020;
    day1 : parse_input_day1 => part1, part2;
    day2 : parse_input_pw_list => part1, part2;
    day3 : get_input_map => part1, part2;
    day4 : parse_input => part1, part2;
    day5 : parse_input => part1, part2, part2_functional;
    day6 : parse_input => part1, part2;
    day7 : parse_input => part1, part2;
    day8 : parse_input => part1, part2;
    day9 : parse_input => part1, part2;
    day10 : parse_input => part1, part2, part2_combinations_recursive_o_n;
    day11 : parse_input => part1, part2;
    day12 : parse_input => part1, part2;
    day13 : parse_input => part1;
}
