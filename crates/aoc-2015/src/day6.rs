use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{newline, space1};
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};

struct Coordinate {
    x: usize,
    y: usize,
}

enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Bounds {
    start: Coordinate,
    end: Coordinate,
}

struct Instruction {
    operation: Operation,
    bounds: Bounds,
}

struct LitGrid {
    lights: Vec<Vec<bool>>,
}

struct BrightnessGrid {
    lights: Vec<Vec<u32>>,
}

trait KernelApply<T> {
    fn lights(&mut self) -> &mut Vec<Vec<T>>;
    fn apply_kernel(&mut self, bounds: &Bounds, op: fn(&mut T)) {
        let start = &bounds.start;
        let end = &bounds.end;
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                op(&mut self.lights()[x][y]);
            }
        }
    }
}

impl KernelApply<bool> for LitGrid {
    fn lights(&mut self) -> &mut Vec<Vec<bool>> {
        &mut self.lights
    }
}

impl KernelApply<u32> for BrightnessGrid {
    fn lights(&mut self) -> &mut Vec<Vec<u32>> {
        &mut self.lights
    }
}

impl LitGrid {
    fn new() -> LitGrid {
        LitGrid {
            lights: vec![vec![false; 1000]; 1000],
        }
    }
    fn apply(&mut self, instructions: &[Instruction]) {
        let op_turn_on = |light: &mut bool| {
            *light = true;
        };
        let op_turn_off = |light: &mut bool| {
            *light = false;
        };
        let op_toggle = |light: &mut bool| {
            *light = !*light;
        };
        for instruction in instructions {
            match instruction.operation {
                Operation::TurnOn => self.apply_kernel(&instruction.bounds, op_turn_on),
                Operation::TurnOff => self.apply_kernel(&instruction.bounds, op_turn_off),
                Operation::Toggle => self.apply_kernel(&instruction.bounds, op_toggle),
            }
        }
    }
    fn total_lit(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().filter(|&&light| light).count())
            .sum()
    }
}

impl BrightnessGrid {
    fn new() -> BrightnessGrid {
        BrightnessGrid {
            lights: vec![vec![0; 1000]; 1000],
        }
    }
    fn apply(&mut self, instructions: &[Instruction]) {
        let op_turn_on = |light: &mut u32| {
            *light += 1;
        };
        let op_turn_off = |light: &mut u32| {
            *light -= 1.min(*light);
        };
        let op_toggle = |light: &mut u32| {
            *light += 2;
        };
        for instruction in instructions {
            match instruction.operation {
                Operation::TurnOn => self.apply_kernel(&instruction.bounds, op_turn_on),
                Operation::TurnOff => self.apply_kernel(&instruction.bounds, op_turn_off),
                Operation::Toggle => self.apply_kernel(&instruction.bounds, op_toggle),
            }
        }
    }
    fn total_brightness(&self) -> u32 {
        self.lights.iter().map(|row| row.iter().sum::<u32>()).sum()
    }
}

fn complete_coordinate(i: &str) -> IResult<&str, Coordinate> {
    separated_pair(complete::u32, tag(","), complete::u32)
        .map(|(x, y)| Coordinate {
            x: x as usize,
            y: y as usize,
        })
        .parse(i)
}

fn parse_operation(i: &str) -> IResult<&str, Operation> {
    alt((
        tag("turn on").map(|_| Operation::TurnOn),
        tag("turn off").map(|_| Operation::TurnOff),
        tag("toggle").map(|_| Operation::Toggle),
    ))
    .parse(i)
}

fn parse_bounds(i: &str) -> IResult<&str, Bounds> {
    separated_pair(
        complete_coordinate,
        (space1, tag("through"), space1),
        complete_coordinate,
    )
    .map(|(start, end)| Bounds { start, end })
    .parse(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    terminated(
        separated_pair(parse_operation, space1, parse_bounds),
        opt(newline),
    )
    .map(|(operation, bounds)| Instruction { operation, bounds })
    .parse(i)
}

fn parse_input(i: &str) -> Result<Vec<Instruction>> {
    many1(parse_instruction).parse(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2015
    }

    fn day(&self) -> Day {
        Day::Day6
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let instructions = parse_input(input)?;
        let mut grid = LitGrid::new();
        grid.apply(&instructions);
        let total = grid.total_lit();
        Ok(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let instruction = parse_input(input)?;
        let mut grid = BrightnessGrid::new();
        grid.apply(&instruction);
        let total = grid.total_brightness();
        Ok(total.to_string())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::*;

    #[fixture]
    fn solver() -> Solver {
        Solver {}
    }

    #[rstest]
    #[case("turn on 0,0 through 999,999", "1000000")]
    #[case("toggle 0,0 through 999,0", "1000")]
    #[case("turn off 499,499 through 500,500", "0")]
    fn example1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("turn on 0,0 through 0,0", "1")]
    #[case("toggle 0,0 through 999,999", "2000000")]
    fn example2(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part2(&input)?, expected);
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "569999");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "17836115");
        Ok(())
    }
}
