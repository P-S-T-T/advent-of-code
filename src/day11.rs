/*
--- Day 11: Seating System ---

Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!

By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).

The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:

If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
Otherwise, the seat's state does not change.
Floor (.) never changes; seats don't move, and nobody sits on the floor.

After one round of these rules, every seat in the example layout becomes occupied:

#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
After a second round, the seats with four or more occupied adjacent seats become empty again:

#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
This process continues for three more rounds:

#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##

#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##

#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.

Simulate_part1 your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?

Your puzzle answer was 2254.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---

As soon as people start to arrive, you realize your mistake. People don't just care about adjacent seats -
they care about the first seat they can see in each of those eight directions!
Now, instead of considering just the eight immediately adjacent seats, consider the first seat in each of
those eight directions. For example, the empty seat below would see eight occupied seats:

.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....
The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:

.............
.L.L.#.#.#.#.
.............
The empty seat below would see no occupied seats:

.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.
Also, people seem to be more tolerant than you expected: it now takes five or more visible occupied seats for an occupied seat to become empty (rather than four or more
from the previous rules). The other rules still apply: empty seats that see no occupied seats become occupied, seats matching no rule don't change, and floor never changes.

Given the same starting layout as above, these new rules cause the seating area to shift around as follows:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL

#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##

#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#

#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#

Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once this occurs, you count 26 occupied seats.

Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied?
2004
*/

type SeatingArea = Vec<Vec<char>>;

const FLOOR: char = '.';
const EMPTY_SEAT: char = 'L';
const OCCUPIED_SEAT: char = '#';

enum Direction {
    West,
    SouthWest,
    South,
    ShouthEast,
    East,
    NorthEast,
    North,
    NorthWest,
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> SeatingArea {
    input.lines().map(|e| e.chars().collect()).collect()
}

#[aoc(day11, part1)]
fn part1(input: &SeatingArea) -> usize {
    let steady_state = simulate_part1(input.clone());
    count_seats(steady_state)
}

#[aoc(day11, part2)]
fn part2(input: &SeatingArea) -> usize {
    let steady_state = simulate_part2(input.clone());
    count_seats(steady_state)
}

fn count_seats(steady_state: Vec<Vec<char>>) -> usize {
    steady_state.iter().flatten().fold(
        0_usize,
        |acc, seat| {
            if *seat == OCCUPIED_SEAT {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn simulate_part1(mut seating_area: SeatingArea) -> SeatingArea {
    let mut steady_state = false;
    while !steady_state {
        let last_state = seating_area.clone();

        // println!();
        // print!("last state:");
        // for row in &last_state {
        //     println!();
        //     for seat in row {
        //         print!("{}", seat);
        //     }
        // }
        // println!();

        steady_state = true;
        for (row_index, row) in last_state.iter().enumerate() {
            for (seat_index, seat) in row.iter().enumerate() {
                match *seat {
                    //Floor (.) never changes; seats don't move, and nobody sits on the floor.
                    FLOOR => continue,
                    //If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
                    EMPTY_SEAT => {
                        if !check_left_righ_any_occupied(seat_index, row) &&
                            // not the first row?
                            (row_index == 0
                                || !check_row_any_occupied(seat_index, &last_state[row_index - 1])) &&
                                // not the lasst row?
                            (row_index == last_state.len()-1
                                || !check_row_any_occupied(seat_index, &last_state[row_index + 1]))
                        {
                            //all seats around are empty
                            steady_state = false;
                            seating_area[row_index][seat_index] = OCCUPIED_SEAT;
                        }
                    }
                    //If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
                    OCCUPIED_SEAT => {
                        let mut occupied_seats_around =
                            check_left_right_number_occupied(seat_index, row);
                        if row_index != 0 {
                            occupied_seats_around +=
                                check_row_number_occupied(seat_index, &last_state[row_index - 1])
                        }
                        if row_index != last_state.len() - 1 {
                            occupied_seats_around +=
                                check_row_number_occupied(seat_index, &last_state[row_index + 1])
                        }
                        if occupied_seats_around >= 4 {
                            steady_state = false;
                            seating_area[row_index][seat_index] = EMPTY_SEAT;
                        }
                    }
                    _ => panic!("unexpected char in puzzle input! '{}'", seat),
                };
            }
        }
    }
    seating_area
}

fn simulate_part2(mut seating_area: SeatingArea) -> SeatingArea {
    let mut steady_state = false;
    while !steady_state {
        let last_state = seating_area.clone();

        // println!();
        // print!("last state:");
        // for row in &last_state {
        //     println!();
        //     for seat in row {
        //         print!("{}", seat);
        //     }
        // }
        // println!();

        steady_state = true;
        for (row_index, row) in last_state.iter().enumerate() {
            for (seat_index, seat) in row.iter().enumerate() {
                match *seat {
                    //Floor (.) never changes; seats don't move, and nobody sits on the floor.
                    FLOOR => continue,
                    //If a seat is empty (L) and there are no occupied seats in the row in all eight directions, the seat becomes occupied.
                    EMPTY_SEAT => {
                        if !neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::West,
                            &last_state,
                        ) && !neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::SouthWest,
                            &last_state,
                        ) && !neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::South,
                            &last_state,
                        ) && !neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::ShouthEast,
                            &last_state,
                        ) && !neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::East,
                            &last_state,
                        ) && !neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::NorthEast,
                            &last_state,
                        ) && !neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::North,
                            &last_state,
                        ) && !neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::NorthWest,
                            &last_state,
                        ) {
                            //all seats around are empty
                            steady_state = false;
                            seating_area[row_index][seat_index] = OCCUPIED_SEAT;
                        }
                    }
                    //If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
                    OCCUPIED_SEAT => {
                        let mut occupied_seats_around = 0_usize;
                        if neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::West,
                            &last_state,
                        ) {
                            occupied_seats_around += 1;
                        };
                        if neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::SouthWest,
                            &last_state,
                        ) {
                            occupied_seats_around += 1;
                        };
                        if neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::South,
                            &last_state,
                        ) {
                            occupied_seats_around += 1;
                        };
                        if neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::ShouthEast,
                            &last_state,
                        ) {
                            occupied_seats_around += 1;
                        };
                        if neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::East,
                            &last_state,
                        ) {
                            occupied_seats_around += 1;
                        };
                        if neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::NorthEast,
                            &last_state,
                        ) {
                            occupied_seats_around += 1;
                        };
                        if neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::North,
                            &last_state,
                        ) {
                            occupied_seats_around += 1;
                        };
                        if neighboring_seat_occupied(
                            seat_index,
                            row_index,
                            Direction::NorthWest,
                            &last_state,
                        ) {
                            occupied_seats_around += 1;
                        };
                        if occupied_seats_around >= 5 {
                            steady_state = false;
                            seating_area[row_index][seat_index] = EMPTY_SEAT;
                        };
                    }
                    _ => panic!("unexpected char in puzzle input! '{}'", seat),
                };
            }
        }
    }
    seating_area
}

fn neighboring_seat_occupied(
    seat_index: usize,
    row_index: usize,
    direction: Direction,
    seating_area: &SeatingArea,
) -> bool {
    match direction {
        Direction::West => {
            if seat_index == seating_area[row_index].len() - 1 {
                false
            } else {
                match seating_area[row_index][seat_index + 1] {
                    OCCUPIED_SEAT => true,
                    EMPTY_SEAT => false,
                    FLOOR => neighboring_seat_occupied(
                        seat_index + 1,
                        row_index,
                        direction,
                        seating_area,
                    ),
                    _ => panic!("Map contains wrong symbol"),
                }
            }
        }
        Direction::South => {
            if row_index == seating_area.len() - 1 {
                false
            } else {
                match seating_area[row_index + 1][seat_index] {
                    OCCUPIED_SEAT => true,
                    EMPTY_SEAT => false,
                    FLOOR => neighboring_seat_occupied(
                        seat_index,
                        row_index + 1,
                        direction,
                        seating_area,
                    ),
                    _ => panic!("Map contains wrong symbol"),
                }
            }
        }
        Direction::East => {
            if seat_index == 0 {
                false
            } else {
                match seating_area[row_index][seat_index - 1] {
                    OCCUPIED_SEAT => true,
                    EMPTY_SEAT => false,
                    FLOOR => neighboring_seat_occupied(
                        seat_index - 1,
                        row_index,
                        direction,
                        seating_area,
                    ),
                    _ => panic!("Map contains wrong symbol"),
                }
            }
        }
        Direction::North => {
            if row_index == 0 {
                false
            } else {
                match seating_area[row_index - 1][seat_index] {
                    OCCUPIED_SEAT => true,
                    EMPTY_SEAT => false,
                    FLOOR => neighboring_seat_occupied(
                        seat_index,
                        row_index - 1,
                        direction,
                        seating_area,
                    ),
                    _ => panic!("Map contains wrong symbol"),
                }
            }
        }
        Direction::SouthWest => {
            if seat_index == seating_area[row_index].len() - 1
                || row_index == seating_area.len() - 1
            {
                false
            } else {
                match seating_area[row_index + 1][seat_index + 1] {
                    OCCUPIED_SEAT => true,
                    EMPTY_SEAT => false,
                    FLOOR => neighboring_seat_occupied(
                        seat_index + 1,
                        row_index + 1,
                        direction,
                        seating_area,
                    ),
                    _ => panic!("Map contains wrong symbol"),
                }
            }
        }
        Direction::NorthWest => {
            if seat_index == seating_area[row_index].len() - 1 || row_index == 0 {
                false
            } else {
                match seating_area[row_index - 1][seat_index + 1] {
                    OCCUPIED_SEAT => true,
                    EMPTY_SEAT => false,
                    FLOOR => neighboring_seat_occupied(
                        seat_index + 1,
                        row_index - 1,
                        direction,
                        seating_area,
                    ),
                    _ => panic!("Map contains wrong symbol"),
                }
            }
        }
        Direction::ShouthEast => {
            if seat_index == 0 || row_index == seating_area.len() - 1 {
                false
            } else {
                match seating_area[row_index + 1][seat_index - 1] {
                    OCCUPIED_SEAT => true,
                    EMPTY_SEAT => false,
                    FLOOR => neighboring_seat_occupied(
                        seat_index - 1,
                        row_index + 1,
                        direction,
                        seating_area,
                    ),
                    _ => panic!("Map contains wrong symbol"),
                }
            }
        }
        Direction::NorthEast => {
            if seat_index == 0 || row_index == 0 {
                false
            } else {
                match seating_area[row_index - 1][seat_index - 1] {
                    OCCUPIED_SEAT => true,
                    EMPTY_SEAT => false,
                    FLOOR => neighboring_seat_occupied(
                        seat_index - 1,
                        row_index - 1,
                        direction,
                        seating_area,
                    ),
                    _ => panic!("Map contains wrong symbol"),
                }
            }
        }
    }
}

fn check_left_occupied(seat_index: usize, row: &[char]) -> bool {
    if seat_index == 0 {
        return false;
    }
    row[seat_index - 1] == OCCUPIED_SEAT
}
fn check_right_occupied(seat_index: usize, row: &[char]) -> bool {
    if seat_index == row.len() - 1 {
        return false;
    }
    row[seat_index + 1] == OCCUPIED_SEAT
}

fn check_left_righ_any_occupied(seat_index: usize, row: &[char]) -> bool {
    check_left_occupied(seat_index, row) || check_right_occupied(seat_index, row)
}

fn check_row_any_occupied(seat_index: usize, row: &[char]) -> bool {
    row[seat_index] == OCCUPIED_SEAT || check_left_righ_any_occupied(seat_index, row)
}

fn check_left_right_number_occupied(seat_index: usize, row: &[char]) -> usize {
    let mut result = 0_usize;
    if check_left_occupied(seat_index, row) {
        result += 1;
    }
    if check_right_occupied(seat_index, row) {
        result += 1;
    }
    result
}

fn check_row_number_occupied(seat_index: usize, row: &[char]) -> usize {
    let mut result = 0_usize;
    if row[seat_index] == OCCUPIED_SEAT {
        result += 1;
    }
    result + check_left_right_number_occupied(seat_index, row)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    #[test]
    fn test_part1_samples() {
        assert_eq!(37, part1(&parse_input(SAMPLE_INPUT_1)));
    }

    #[test]
    fn test_part2_sample_1() {
        assert_eq!(26, part2(&parse_input(SAMPLE_INPUT_1)));
    }
}
