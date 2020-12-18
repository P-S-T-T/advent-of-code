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
fn main() {
  let expense_report = [
    1975, 1446, 1902, 1261, 1783, 1535, 1807, 1606, 1685, 1933, 1930, 1813, 1331, 1986, 1379, 1649,
    1342, 1206, 1832, 1464, 1840, 1139, 1316, 1366, 593, 1932, 1553, 1065, 2004, 1151, 1345, 1026,
    1958, 1778, 1987, 1425, 1170, 1927, 1487, 1116, 1612, 2005, 1977, 1691, 1964, 398, 1621, 1542,
    1929, 1102, 1993, 1426, 1349, 1280, 1775, 849, 1344, 1940, 1707, 1562, 1979, 1325, 1610, 559,
    1812, 1938, 1572, 1949, 1136, 161, 1893, 1207, 1363, 1551, 1333, 1904, 1332, 1450, 1773, 1216,
    1185, 1881, 1835, 1460, 1277, 1374, 1568, 1731, 1365, 1719, 1749, 1371, 1602, 1108, 1030, 1859,
    1875, 1976, 1837, 1768, 1873, 1226, 1533, 1601, 1394, 1422, 1219, 1269, 1793, 1195, 1234, 1575,
    1882, 1223, 1826, 521, 1161, 1738, 1506, 1574, 1337, 1509, 1430, 1496, 1318, 1400, 1852, 1670,
    1898, 1858, 1950, 1870, 1920, 868, 1814, 1853, 1911, 1907, 1713, 1281, 1759, 1210, 1350, 1035,
    1585, 1765, 1220, 1125, 1714, 1810, 1002, 1356, 1192, 1452, 1236, 1482, 1716, 1681, 1323, 1923,
    1876, 1792, 1346, 1891, 1721, 1056, 1675, 1518, 1540, 1068, 1563, 1942, 1668, 1653, 1357, 1632,
    1128, 1726, 1586, 1998, 1138, 1510, 1022, 1480, 1434, 1305, 1861, 1623, 1009, 1339, 1159, 1085,
    1578, 1689, 1091, 1874, 1043, 1737, 1704, 1515,
  ];

  println!("Answer of Part 1 is: {}", part1(&expense_report));
  println!("Answer of Part 2 is: {}", part2(&expense_report));
}

#[aoc_generator(day1)]
fn parse_input(input: &[i32]) -> &[i32] {
  input
}

#[aoc(day2, part1)]
fn part1(expense_report: &[i32]) -> i32 {
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

#[aoc(day2, part2)]
fn part2(expense_report: &[i32]) -> i32 {
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
    if result > 2020 {
      biggest_index -= 1;
    } else if result < 2020 {
      smallest_index += 1;
      third_number_index += 1;
    } else {
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

  println!("No Result, wrong input!");
  0
}
