use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::character::complete;
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::terminated;

struct Bank {
    batteries: Vec<u64>,
}

impl Bank {
    fn new(batteries: Vec<u64>) -> Bank {
        Bank { batteries }
    }
    fn joltage(&self) -> Option<u64> {
        self.batteries
            .iter()
            .tuple_combinations()
            .map(|(a, b)| a * 10 + b)
            .max()
    }
    fn unrestricted_joltage(&self, num: usize) -> Option<u64> {
        let mut digits = String::new();

        let len = self.batteries.len();
        let mut lower_index = 0;
        let mut upper_index = len - num;

        for i in 0..num {
            let (max_index, max_digit) = self
                .batteries
                .iter()
                .enumerate()
                .skip(lower_index)
                .take(upper_index - lower_index + 1)
                .rev()
                .max_by_key(|&(_, &d)| d)?;
            lower_index = max_index + 1;
            upper_index = len - num + i + 1;
            let char = std::char::from_digit(*max_digit as u32, 10)?;
            digits.push(char);
        }

        digits.parse().ok()
    }
}

fn parse_bank(i: &str) -> IResult<&str, Bank> {
    terminated(
        map(complete::digit1, |v: &str| {
            v.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect_vec()
        }),
        opt(complete::newline),
    )
    .map(Bank::new)
    .parse(i)
}

fn parse_input(input: &str) -> Result<Vec<Bank>> {
    many1(parse_bank).parse(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2025
    }

    fn day(&self) -> Day {
        Day::Day3
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let banks = parse_input(input.trim())?;
        let sum: u64 = banks.iter().map(|bank| bank.joltage().unwrap_or(0)).sum();
        Ok(sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let banks = parse_input(input.trim())?;
        let sum: u64 = banks
            .iter()
            .map(|bank| bank.unrestricted_joltage(12).unwrap_or(0))
            .sum();
        Ok(sum.to_string())
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
    #[case("987654321111111", "98")]
    #[case("811111111111119", "89")]
    #[case("234234234234278", "78")]
    #[case("818181911112111", "92")]
    fn cases1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("987654321111111", "987654321111")]
    #[case("811111111111119", "811111111119")]
    #[case("234234234234278", "434234234278")]
    #[case("818181911112111", "888911112111")]
    fn cases2(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part2(&input)?, expected);
        Ok(())
    }

    #[rstest]
    fn example1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example1)?;
        assert_eq!(solver.solve_part1(&input)?, "357");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "3121910778619");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "16946");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "168627047606506");
        Ok(())
    }
}
