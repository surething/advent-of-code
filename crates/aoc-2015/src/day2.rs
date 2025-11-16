use nom::character::complete;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::terminated;
use aoc_common::prelude::*;
use aoc_data::prelude::*;

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(length: u32, width: u32, height: u32) -> Present {
        Present {
            length,
            width,
            height,
        }
    }
    fn surface_area(&self) -> u32 {
        let area_1 = self.length * self.width;
        let area_2 = self.width * self.height;
        let area_3 = self.height * self.length;

        let smallest_area = area_1.min(area_2).min(area_3);

        2 * (area_1 + area_2 + area_3) + smallest_area
    }
    fn ribbon_length(&self) -> u32 {
        let mut dimensions = vec![self.length, self.width, self.height];
        dimensions.sort_unstable();
        let perimeter = 2 * (dimensions[0] + dimensions[1]);
        let bow = self.length * self.width * self.height;
        perimeter + bow
    }
}

fn parse_present(i: &str) -> IResult<&str, Present> {
        terminated(
            (
                complete::u32,
                complete::char('x'),
                complete::u32,
                complete::char('x'),
                complete::u32,
            ),
            opt(complete::newline),
        )
        .map(|(l, _, w, _, h)| Present::new(l, w, h))
        .parse(i)
}

fn parse_input(i: &str) -> Result<Vec<Present>> {
    many1(parse_present).parse(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2015
    }

    fn day(&self) -> Day {
        Day::Day2
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let presents = parse_input(input.trim())?;
        let total_area: u32 = presents.iter().map(|p| p.surface_area()).sum();
        Ok(total_area.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let presents = parse_input(input.trim())?;
        let total_ribbon: u32 = presents.iter().map(|p| p.ribbon_length()).sum();
        Ok(total_ribbon.to_string())
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
    #[case("2x3x4", "58")]
    #[case("1x1x10", "43")]
    fn example1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("2x3x4", "34")]
    #[case("1x1x10", "14")]
    fn example2(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part2(&input)?, expected);
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "1586300");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "3737498");
        Ok(())
    }
}
