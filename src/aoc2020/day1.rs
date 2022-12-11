/*
--- Day 1: Report Repair ---

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456

In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?

Your puzzle answer was 299299.

--- Part Two ---

The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?

Your puzzle answer was 287730716.
*/
// use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

pub fn parse_input_day1(input: &str) -> Vec<usize> {
  input
    .lines()
    .map(|v| v.parse().expect("Failed to parse input!"))
    .collect()
}

pub fn part1(expense_report: &[usize]) -> usize {
  for compare_index in 1..expense_report.len() - 1 {
    for current in expense_report.iter() {
      if (expense_report[compare_index] + current) == 2020 {
        println!(
          "Found result: {} * {} = {}",
          expense_report[compare_index],
          current,
          expense_report[compare_index] * current
        );
        return expense_report[compare_index] * current;
      }
    }
  }
  println!("No Result, wrong input!");
  0
}

pub fn part2(expense_report: &[usize]) -> usize {
  let mut sorted_expense_report = expense_report.to_owned();
  sorted_expense_report.sort_unstable();

  let mut smallest_index = 0;
  let mut biggest_index = sorted_expense_report.len() - 1;
  let mut third_number_index = 1;
  let mut result;

  while third_number_index < biggest_index {
    result = sorted_expense_report[smallest_index]
      + sorted_expense_report[third_number_index]
      + sorted_expense_report[biggest_index];

    match result.cmp(&2020) {
      Ordering::Greater => biggest_index -= 1,
      Ordering::Less => {
        smallest_index += 1;
        third_number_index += 1;
      }
      Ordering::Equal => {
        println!(
          "Found result! {} + {} + {} = {}, so {} * {} * {} = {}",
          sorted_expense_report[smallest_index],
          sorted_expense_report[third_number_index],
          sorted_expense_report[biggest_index],
          sorted_expense_report[smallest_index]
            + sorted_expense_report[third_number_index]
            + sorted_expense_report[biggest_index],
          sorted_expense_report[smallest_index],
          sorted_expense_report[third_number_index],
          sorted_expense_report[biggest_index],
          sorted_expense_report[smallest_index]
            * sorted_expense_report[third_number_index]
            * sorted_expense_report[biggest_index]
        );
        return sorted_expense_report[smallest_index]
          * sorted_expense_report[third_number_index]
          * sorted_expense_report[biggest_index];
      }
    }
  }

  println!("No Result, wrong input!");
  0
}
