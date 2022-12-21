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

Your puzzle answer was 1010.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---

Before you can give the destination to the captain, you realize that the actual action meanings were printed on the back of the instructions the whole time.

Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:

Action N means to move the waypoint north by the given value.
Action S means to move the waypoint south by the given value.
Action E means to move the waypoint east by the given value.
Action W means to move the waypoint west by the given value.
Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
Action F means to move forward to the waypoint a number of times equal to the given value.

The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.

For example, using the same instructions as above:

F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the
ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.
After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.

Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?

52742
*/

#[derive(Clone)]
pub enum Direction {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}
pub enum Instruction {
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

        let direction = ((*self).clone() as isize + turns) % 4;

        match direction {
            0 => Self::East,
            -3 | 1 => Self::South,
            -2 | 2 => Self::West,
            -1 | 3 => Self::North,
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

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::new).collect()
}

pub fn part1(input: &[Instruction]) -> isize {
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

pub fn part2(input: &[Instruction]) -> isize {
    let mut waypoint_position = Position {
        north: 1,
        east: 10,
        facing: Direction::East,
    };
    let mut ship_position = Position {
        north: 0,
        east: 0,
        facing: Direction::East,
    };
    execute_instructions_part2(&mut ship_position, &mut waypoint_position, input);
    calculate_manhattan_distance(&ship_position)
}

fn execute_instructions_part2(
    ship_position: &mut Position,
    waypoint_position: &mut Position,
    instructions: &[Instruction],
) {
    for instruction in instructions {
        if let Instruction::Forward(units) = instruction {
            ship_position.north += waypoint_position.north * units;
            ship_position.east += waypoint_position.east * units;
        } else {
            execute_instruction_part2(instruction, waypoint_position);
        }
    }
}
fn execute_instruction_part2(instruction: &Instruction, position: &mut Position) {
    match instruction {
        Instruction::Move(direction, units) => match direction {
            Direction::West => position.east -= *units,
            Direction::South => position.north -= *units,
            Direction::East => position.east += *units,
            Direction::North => position.north += *units,
        },

        Instruction::TurnRight(degrees) => turn_waypoint(*degrees, position),
        Instruction::TurnLeft(degrees) => turn_waypoint(-degrees, position),

        Instruction::Forward(_) => {
            panic!("Forward should have been handled in the if-let statement before!!")
        }
    }
}

fn turn_waypoint(degrees: isize, position: &mut Position) {
    if degrees % 90 > 0 {
        panic!("not only right angles! Degree is {}.", degrees)
    }
    let turns = degrees / 90 % 4;
    match turns {
        0 => {} //return position,
        -3 | 1 => {
            let north = position.north;
            position.north = -position.east;
            position.east = north;
        }
        -2 | 2 => {
            position.north = -position.north;
            position.east = -position.east;
        }
        -1 | 3 => {
            let north = position.north;
            position.north = position.east;
            position.east = -north;
        }
        _ => panic!("Impossible angle!"),
    }
}

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
    const SAMPLE_INPUT_2: &str = "F10
R270
F10
";

    #[test]
    fn test_part1_sample_2() {
        assert_eq!(20, part1(&parse_input(SAMPLE_INPUT_2)));
    }
    const SAMPLE_INPUT_3: &str = "F10
R540
F10
";

    #[test]
    fn test_part1_sample_3() {
        assert_eq!(0, part1(&parse_input(SAMPLE_INPUT_3)));
    }
    const SAMPLE_INPUT_4: &str = "F10
L270
F10
";

    #[test]
    fn test_part1_sample_4() {
        assert_eq!(20, part1(&parse_input(SAMPLE_INPUT_4)));
    }
    const SAMPLE_INPUT_5: &str = "F10
R540
F10
";

    #[test]
    fn test_part1_sample_5() {
        assert_eq!(0, part1(&parse_input(SAMPLE_INPUT_5)));
    }

    #[test]
    fn test_part2_sample_1() {
        assert_eq!(286, part2(&parse_input(SAMPLE_INPUT_1)));
    }
}
