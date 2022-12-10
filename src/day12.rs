/*
--- Day 12: Rain Risk ---

Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!

Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.

The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:

Action N means to move north by the given value.
Action S means to move south by the given value.
Action E means to move east by the given value.
Action W means to move west by the given value.
Action L means to turn left the given number of degrees.
Action R means to turn right the given number of degrees.
Action F means to move forward by the given value in the direction the ship is currently facing.
The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)

For example:

F10
N3
F7
R90
F11
These instructions would be handled as follows:

F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
N3 would move the ship 3 units north to east 10, north 3.
F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
F11 would move the ship 11 units south to east 17, south 8.
At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its
starting position is 17 + 8 = 25.

Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?
*/

#[derive(Clone)]
enum Direction {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}
enum Instruction {
    Move(Direction, isize),
    TurnLeft(isize),
    TurnRight(isize),
    Forward(isize),
}

impl Direction {
    fn turn(&self, degrees: &isize) -> Self {
        if degrees % 90 != 0 {
            panic!("Angle not 90 deg! {}", degrees)
        };
        let ticks = degrees / 90;
        let turns = ticks % 4;

        let direction = (*self as isize + turns) % 4;

        match direction {
            0 => Self::West,
            1 => Self::South,
            2 => Self::East,
            3 => Self::North,
            _ => panic!("impossible!"),
        }
    }
}

impl Instruction {
    fn new(instruction: &str) -> Self {
        let direction = &instruction[0..1];
        let units = instruction[1..].parse().unwrap();

        match direction {
            "N" => Self::Move(Direction::North, units),
            "W" => Self::Move(Direction::West, units),
            "S" => Self::Move(Direction::South, units),
            "E" => Self::Move(Direction::East, units),
            "L" => Self::TurnLeft(units),
            "R" => Self::TurnRight(units),
            "F" => Self::Forward(units),
            _ => panic!("Wrong direction in instruction: {}", direction),
        }
    }
}

#[derive(Clone)]
struct Position {
    north: isize,
    east: isize,
    facing: Direction,
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::new).collect()
}

#[aoc(day12, part1)]
fn part1(input: &[Instruction]) -> isize {
    let initial_position = Position {
        north: 0,
        east: 0,
        facing: Direction::East,
    };
    let position = execute_instructions(&initial_position, input);
    calculate_manhattan_distance(&position)
}

fn execute_instructions(position: &Position, instructions: &[Instruction]) -> Position {
    let mut current_position = position.clone();

    for instruction in instructions {
        if let Instruction::Forward(units) = instruction {
            execute_instruction(
                &Instruction::Move(current_position.facing.clone(), *units),
                &mut current_position,
            )
        } else {
            execute_instruction(instruction, &mut current_position);
        }
    }
    current_position
}

fn execute_instruction(instruction: &Instruction, position: &mut Position) {
    match instruction {
        Instruction::Move(direction, units) => match direction {
            Direction::West => position.east -= *units,
            Direction::South => position.north -= *units,
            Direction::East => position.east += *units,
            Direction::North => position.north += *units,
        },
        Instruction::TurnLeft(degrees) => position.facing = position.facing.turn(&(-*degrees)),
        Instruction::TurnRight(degrees) => position.facing = position.facing.turn(degrees),
        Instruction::Forward(_) => {
            panic!("Forward should have been handled in the if-let statement before!!")
        }
    }
}

fn calculate_manhattan_distance(position: &Position) -> isize {
    position.east.abs() + position.north.abs()
}

// #[aoc(day12, part2)]
// fn part2(input: &SeatingArea) -> usize {
//     let steady_state = simulate_part2(input.clone());
//     count_seats(steady_state)
// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "F10
N3
F7
R90
F11
";

    #[test]
    fn test_part1_samples() {
        assert_eq!(25, part1(&parse_input(SAMPLE_INPUT_1)));
    }

    // #[test]
    // fn test_part2_sample_1() {
    //     assert_eq!(26, part2(&parse_input(SAMPLE_INPUT_1)));
    // }
}
