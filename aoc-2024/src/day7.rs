use aoc_common::prelude::*;
use aoc_data::prelude::*;
use itertools::{Either, Itertools};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{newline, space1};
use nom::combinator::opt;
use nom::multi::{many1, separated_list1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Sum,
    Mul,
    Join,
}

#[derive(Debug)]
struct Equation {
    lhs: i64,
    rhs: Vec<i64>,
}

impl Equation {
    fn all_calculations(&self, ops: &[Op]) -> Vec<Calculation> {
        // Permutations with replacement.
        let i = ops.iter().cloned();
        let k = self.rhs.len() - 1;
        itertools::repeat_n(i, k)
            .multi_cartesian_product()
            .map(|ops| {
                let values = self.rhs.clone();
                Calculation { ops, values }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Calculation {
    ops: Vec<Op>,
    values: Vec<i64>,
}

fn reduce(calc: Calculation) -> Either<Calculation, i64> {
    match calc.values.as_slice() {
        [] => Either::Right(0),
        [value] => Either::Right(*value),
        _ => {
            let mut new_calc = calc.clone();
            match calc.ops[0] {
                Op::Mul => new_calc.values[1] *= new_calc.values[0],
                Op::Sum => new_calc.values[1] += new_calc.values[0],
                Op::Join => {
                    let n = (new_calc.values[1] as f64).log10().floor() as u32 + 1;
                    new_calc.values[1] += 10_i64.pow(n) * new_calc.values[0];
                }
            }
            new_calc.values.remove(0);
            new_calc.ops.remove(0);
            Either::Left(new_calc)
        }
    }
}

impl Calculation {
    fn eval(&self) -> i64 {
        let mut calc = self.clone();
        loop {
            calc = match reduce(calc) {
                Either::Right(value) => break value,
                Either::Left(new_calc) => new_calc,
            };
        }
    }
}

type Equations = Vec<Equation>;

fn parse_equation(i: &str) -> IResult<&str, Equation> {
    let (i, lhs) = complete::i64(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, rhs) = separated_list1(space1, complete::i64)(i)?;
    let (i, _) = opt(newline)(i)?;
    Ok((i, Equation { lhs, rhs }))
}

fn parse_input(i: &str) -> Result<Equations> {
    many1(parse_equation)(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day7
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let equations = parse_input(input)?;
        let result: i64 = equations
            .iter()
            .filter(|eq| {
                eq.all_calculations(&[Op::Sum, Op::Mul])
                    .iter()
                    .any(|calc| calc.eval() == eq.lhs)
            })
            .map(|eq| eq.lhs)
            .sum();
        Ok(result.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let equations = parse_input(input)?;
        let result: i64 = equations
            .iter()
            .filter(|eq| {
                eq.all_calculations(&[Op::Sum, Op::Mul, Op::Join])
                    .iter()
                    .any(|calc| calc.eval() == eq.lhs)
            })
            .map(|eq| eq.lhs)
            .sum();
        Ok(result.to_string())
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
    fn example1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example1)?;
        assert_eq!(solver.solve_part1(&input)?, "3749");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "11387");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "20665830408335");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "354060705047464");
        Ok(())
    }
}
