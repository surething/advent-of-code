use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::character::complete;
use nom::character::complete::newline;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::terminated;

fn calculate_fuel(mass: &i32) -> i32 {
    mass / 3 - 2
}

fn calculate_full_fuel(mass: &i32) -> i32 {
    let mut total_fuel = 0;
    let mut additional_fuel = calculate_fuel(mass);
    while additional_fuel > 0 {
        total_fuel += additional_fuel;
        additional_fuel = calculate_fuel(&additional_fuel);
    }
    total_fuel
}

fn parse_mass(i: &str) -> IResult<&str, i32> {
    terminated(complete::i32, opt(newline)).parse(i)
}

fn parse_input(i: &str) -> Result<Vec<i32>> {
    many1(parse_mass).parse(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2019
    }

    fn day(&self) -> Day {
        Day::Day1
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let masses = parse_input(input.trim())?;
        let fuel = masses.iter().map(calculate_fuel).sum::<i32>();
        Ok(fuel.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let masses = parse_input(input.trim())?;
        let fuel = masses.iter().map(calculate_full_fuel).sum::<i32>();
        Ok(fuel.to_string())
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
    #[case("12", "2")]
    #[case("14", "2")]
    #[case("1969", "654")]
    #[case("100756", "33583")]
    fn example1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("14", "2")]
    #[case("1969", "966")]
    #[case("100756", "50346")]
    fn example2(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part2(&input)?, expected);
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "3426455");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "5136807");
        Ok(())
    }
}
