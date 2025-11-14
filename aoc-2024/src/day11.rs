use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::character::complete;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::multi::separated_list1;
use std::collections::HashMap;

type Stone = u64;
type Stones = HashMap<Stone, usize>;

trait StonesExt {
    fn blink(&self) -> Stones;
}

impl StonesExt for Stones {
    fn blink(&self) -> Stones {
        let mut stones = HashMap::new();
        for (stone, count) in self {
            match stone {
                0 => *stones.entry(1).or_insert(0) += count,
                s if s.ilog10() % 2 == 1 => {
                    let d = s.ilog10() / 2 + 1;
                    let a = stone / 10u64.pow(d);
                    let b = stone % 10u64.pow(d);
                    *stones.entry(a).or_insert(0) += count;
                    *stones.entry(b).or_insert(0) += count;
                }
                s => *stones.entry(s * 2024).or_insert(0) += count,
            }
        }
        stones
    }
}

fn parse_stones(i: &str) -> IResult<&str, Stones> {
    map(separated_list1(space1, complete::u64), |stones| {
        stones.into_iter().fold(HashMap::new(), |mut acc, stone| {
            *acc.entry(stone).or_insert(0) += 1;
            acc
        })
    })(i)
}

fn parse_input(input: &str) -> Result<Stones> {
    parse_stones(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day11
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let mut stones = parse_input(input)?;
        for _ in 0..25 {
            stones = stones.blink();
        }
        Ok(stones.values().sum::<usize>().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let mut stones = parse_input(input)?;
        for _ in 0..75 {
            stones = stones.blink();
        }
        Ok(stones.values().sum::<usize>().to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "55312");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "65601038650482");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "199753");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "239413123020116");
        Ok(())
    }
}
