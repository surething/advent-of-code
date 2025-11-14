use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nalgebra::{Matrix2, Vector2};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{anychar, newline};
use nom::combinator::opt;
use nom::multi::separated_list1;

struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

struct Button {
    dx: i64,
    dy: i64,
}

impl Button {
    fn new(dx: i64, dy: i64) -> Self {
        Self { dx, dy }
    }
}

struct Prize {
    loc: Coordinate,
}

impl Prize {
    fn new(loc: Coordinate) -> Self {
        Self { loc }
    }
}

struct Machine {
    a: Button,
    b: Button,
    prize: Prize,
}

#[derive(Debug)]
struct Moves {
    a: i64,
    b: i64,
}

impl Moves {
    fn cost(&self) -> i64 {
        3 * self.a + self.b
    }
}

impl Machine {
    fn show(&self) {
        println!("Button A: X+{}, Y+{}", self.a.dx, self.a.dy);
        println!("Button B: X+{}, Y+{}", self.b.dx, self.b.dy);
        println!("Prize: X={}, Y={}", self.prize.loc.x, self.prize.loc.y);
    }
    fn all_possible_moves(&self) -> Vec<Moves> {
        let mut moves = vec![];
        let (ax, ay) = (self.a.dx, self.a.dy);
        let (bx, by) = (self.b.dx, self.b.dy);
        let (px, py) = (self.prize.loc.x, self.prize.loc.y);

        let max_a = px / ax;
        let max_b = py / by;

        for a in 0..=max_a {
            for b in 0..=max_b {
                if (a * ax + b * bx == px) && (a * ay + b * by == py) {
                    moves.push(Moves { a, b });
                }
            }
        }
        moves
    }
    fn move_prize(&mut self, dx: i64, dy: i64) {
        self.prize.loc.x += dx;
        self.prize.loc.y += dy;
    }
    fn cheapest_move(&self) -> Option<Moves> {
        //
        // Suffers in brute-force searching.
        //
        self.show();
        self.all_possible_moves()
            .into_iter()
            .min_by_key(|m| m.cost())
    }
    fn cheapest_move_numerical(&self) -> Option<Moves> {
        //
        // Suffers in numerical instability.
        //
        let a = Matrix2::new(
            self.a.dx as f64,
            self.b.dx as f64,
            self.a.dy as f64,
            self.b.dy as f64,
        );
        let mut b = Vector2::new(self.prize.loc.x as f64, self.prize.loc.y as f64);
        let lu = a.lu();
        let tol = 1e-12;
        match lu.solve(&b) {
            Some(x)
                if x.iter()
                    .all(|v| v.fract().abs() < tol || (1.0 - v.fract()).abs() < tol) =>
            {
                Some(Moves {
                    a: x[0].round() as i64,
                    b: x[1].round() as i64,
                })
            }
            Some(_) | None => None,
        }
    }
    fn cheapest_move_linear_algebra(&self) -> Option<Moves> {
        let (ax, ay) = (self.a.dx, self.a.dy);
        let (bx, by) = (self.b.dx, self.b.dy);
        let (px, py) = (self.prize.loc.x, self.prize.loc.y);

        let det = ax * by - ay * bx;

        let na = (by * px - bx * py) as f64 / (det as f64);
        let nb = (ax * py - ay * px) as f64 / (det as f64);

        fn is_integer(x: f64, tol: f64) -> Option<i64> {
            if (x.fract().abs() < tol) || (1.0 - x.fract()).abs() < tol {
                Some(x.round() as i64)
            } else {
                None
            }
        }
        let tol = 1e-12;
        is_integer(na, tol).and_then(|a| is_integer(nb, tol).map(|b| Moves { a, b }))
    }
}

type Machines = Vec<Machine>;

fn parse_button(i: &str) -> IResult<&str, Button> {
    let (i, _) = tag("Button ").parse(i)?;
    let (i, _) = anychar(i)?;
    let (i, _) = tag(": X+").parse(i)?;
    let (i, dx) = complete::i64(i)?;
    let (i, _) = tag(", Y+").parse(i)?;
    let (i, dy) = complete::i64(i)?;
    let (i, _) = opt(newline).parse(i)?;
    Ok((i, Button::new(dx, dy)))
}

fn parse_prize(i: &str) -> IResult<&str, Prize> {
    let (i, _) = tag("Prize: ").parse(i)?;
    let (i, _) = tag("X=").parse(i)?;
    let (i, x) = complete::i64(i)?;
    let (i, _) = tag(", Y=").parse(i)?;
    let (i, y) = complete::i64(i)?;
    let (i, _) = opt(newline).parse(i)?;
    Ok((i, Prize::new(Coordinate::new(x, y))))
}
fn parse_machine(i: &str) -> IResult<&str, Machine> {
    let (i, a) = parse_button(i)?;
    let (i, b) = parse_button(i)?;
    let (i, prize) = parse_prize(i)?;
    Ok((i, Machine { a, b, prize }))
}

fn parse_machines(i: &str) -> IResult<&str, Machines> {
    separated_list1(newline, parse_machine).parse(i)
}

fn parse_input(input: &str) -> Result<Machines> {
    parse_machines(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day13
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let machines = parse_input(input)?;
        let lowest_cost: i64 = machines
            .iter()
            .flat_map(|m| m.cheapest_move_linear_algebra())
            .map(|m| m.cost())
            .sum();
        Ok(lowest_cost.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let mut machines = parse_input(input)?;

        let dx = 10_000_000_000_000_i64;
        let dy = 10_000_000_000_000_i64;
        machines.iter_mut().for_each(|m| m.move_prize(dx, dy));

        let lowest_cost: i64 = machines
            .iter()
            .flat_map(|m| m.cheapest_move_linear_algebra())
            .map(|m| m.cost())
            .sum();

        Ok(lowest_cost.to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "480");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "875318608908");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "28887");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "96979582619758");
        Ok(())
    }
}
