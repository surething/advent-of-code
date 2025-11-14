use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{newline, space1};
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::separated_pair;

#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct State {
    pos: Vec2,
    vel: Vec2,
}

type States = Vec<State>;

struct Arena {
    width: i32,
    height: i32,
    states: States,
}

impl Arena {
    fn new(width: i32, height: i32, states: States) -> Arena {
        assert_eq!(width % 2, 1);
        assert_eq!(height % 2, 1);
        Arena {
            width,
            height,
            states,
        }
    }
    fn show(&self) {
        let mut grid = vec![vec![' '; self.width as usize]; self.height as usize];
        for state in &self.states {
            grid[state.pos.y as usize][state.pos.x as usize] = 'x';
        }
        for row in grid {
            println!(
                "{} ",
                row.into_iter().flat_map(|c| [c, ' ']).collect::<String>()
            );
        }
    }
    fn tick(&mut self) {
        self.states.iter_mut().for_each(|state| {
            state.pos.x += state.vel.x + self.width;
            state.pos.y += state.vel.y + self.height;
            state.pos.x %= self.width;
            state.pos.y %= self.height;
        })
    }
    fn safety_factor(&self) -> usize {
        // Note - the middle row / column is ignored.

        let h = self.height;
        let w = self.width;

        let rows_0 = 0..(h / 2);
        let rows_1 = ((h + 1) / 2)..h;

        let cols_0 = 0..(w / 2);
        let cols_1 = ((w + 1) / 2)..w;

        let quadrant_0 = (&rows_0, &cols_0);
        let quadrant_1 = (&rows_0, &cols_1);
        let quadrant_2 = (&rows_1, &cols_0);
        let quadrant_3 = (&rows_1, &cols_1);

        let quadrants = [quadrant_0, quadrant_1, quadrant_2, quadrant_3];

        quadrants
            .iter()
            .map(|(rows, cols)| {
                self.states
                    .iter()
                    .filter(|state| rows.contains(&state.pos.y) && cols.contains(&state.pos.x))
                    .count()
            })
            .product()
    }
    fn entropy(&self, subgrid_size: (usize, usize)) -> f64 {
        assert!(subgrid_size.0 <= self.width as usize);
        assert!(subgrid_size.1 <= self.height as usize);
        let (subgrid_width, subgrid_height) = subgrid_size;
        let subgrid_area = subgrid_width * subgrid_height;
        let mut entropy = 0.0;
        for y in 0..(self.height as usize - subgrid_height) {
            for x in 0..(self.width as usize - subgrid_width) {
                // TODO: This has terrible performance, and could be optimized.
                let subgrid = self
                    .states
                    .iter()
                    .filter(|state| {
                        (y..(y + subgrid_height)).contains(&(state.pos.y as usize))
                            && (x..(x + subgrid_width)).contains(&(state.pos.x as usize))
                    })
                    .count();
                entropy += match subgrid {
                    0 => 0.0,
                    _ => {
                        let p = (subgrid as f64) / (subgrid_area as f64);
                        -p * p.log2()
                    }
                };
            }
        }
        entropy
    }
}

fn parse_vec2(i: &str) -> IResult<&str, Vec2> {
    let (i, (x, y)) = separated_pair(complete::i32, tag(","), complete::i32).parse(i)?;
    Ok((i, Vec2 { x, y }))
}

fn parse_state(i: &str) -> IResult<&str, State> {
    let (i, _) = tag("p=").parse(i)?;
    let (i, pos) = parse_vec2(i)?;
    let (i, _) = space1(i)?;
    let (i, _) = tag("v=").parse(i)?;
    let (i, vel) = parse_vec2(i)?;
    let (i, _) = opt(newline).parse(i)?;
    Ok((i, State { pos, vel }))
}

fn parse_input(i: &str) -> Result<Vec<State>> {
    many1(parse_state).parse(i).map_and_finish()
}

struct Solver {
    arena_size: (i32, i32),
}

impl Solver {
    fn make_arena(&self, states: States) -> Arena {
        Arena::new(self.arena_size.0, self.arena_size.1, states)
    }
}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day14
    }
    fn solve_part1(&self, input: &str) -> Result<String> {
        let states = parse_input(input)?;
        let mut arena = self.make_arena(states);
        for _ in 0..100 {
            arena.tick();
        }
        Ok(arena.safety_factor().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let states = parse_input(input)?;
        let mut arena = self.make_arena(states);
        for t in 1..10_000 {
            arena.tick();
            let e = arena.entropy((2, 2));
            // TODO: this is hard-coded, and could be replaced by detecting a
            //       statistically significant change in entropy.
            if e < 500.0 {
                // arena.show();
                return Ok(t.to_string());
            }
        }
        Err(AdventError::Other("Limit reached".to_string()))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::*;

    #[rstest]
    fn example1() -> Result<()> {
        let solver = Solver {
            arena_size: (11, 7),
        };
        let input = solver.read_resource(Input::Example1)?;
        assert_eq!(solver.solve_part1(&input)?, "12");
        Ok(())
    }

    #[rstest]
    fn part1() -> Result<()> {
        let solver = Solver {
            arena_size: (101, 103),
        };
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "208437768");
        Ok(())
    }

    #[rstest]
    fn part2() -> Result<()> {
        let solver = Solver {
            arena_size: (101, 103),
        };
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "7492");
        Ok(())
    }
}
