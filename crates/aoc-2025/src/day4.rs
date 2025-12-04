use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::branch::alt;
use nom::character::complete;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::terminated;
use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Debug)]
enum Tile {
    Empty,
    Roll,
}

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Delta {
    dx: isize,
    dy: isize,
}

impl Delta {
    fn new(dx: isize, dy: isize) -> Delta {
        Delta { dx, dy }
    }
    fn adjacent() -> impl Iterator<Item = Delta> {
        let deltas: [(isize, isize); 8] = [
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            //(0, 0), // skip self
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        deltas.into_iter().map(Delta::from)
    }
}

impl<T, S> From<(T, S)> for Delta
where
    T: Into<isize>,
    S: Into<isize>,
{
    fn from(value: (T, S)) -> Self {
        Delta {
            dx: value.0.into(),
            dy: value.1.into(),
        }
    }
}

impl Add<&Delta> for &Coordinate {
    type Output = Option<Coordinate>;

    fn add(self, delta: &Delta) -> Self::Output {
        let new_x = self.x as isize + delta.dx;
        let new_y = self.y as isize + delta.dy;
        if new_x >= 0 && new_y >= 0 {
            Some(Coordinate::new(new_x as usize, new_y as usize))
        } else {
            None
        }
    }
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
    fn neighbors(&self) -> impl Iterator<Item = Coordinate> {
        Delta::adjacent().filter_map(move |d| self + &d)
    }
}

impl<T, S> From<(T, S)> for Coordinate
where
    T: Into<usize>,
    S: Into<usize>,
{
    fn from(value: (T, S)) -> Self {
        Coordinate {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(tiles: Vec<Vec<Tile>>) -> Grid {
        let height = tiles.len();
        let width = if height > 0 { tiles[0].len() } else { 0 };
        Grid {
            tiles,
            width,
            height,
        }
    }
    fn get(&self, coord: &Coordinate) -> Option<&Tile> {
        if coord.y < self.height && coord.x < self.width {
            Some(&self.tiles[coord.y][coord.x])
        } else {
            None
        }
    }
    fn neighbors(&self, coord: &Coordinate) -> impl Iterator<Item = (Coordinate, &Tile)> {
        coord
            .neighbors()
            .filter(|c| c.x < self.width && c.y < self.height)
            .filter_map(move |c| self.get(&c).map(|t| (c, t)))
    }
    fn coordinates(&self) -> impl Iterator<Item = Coordinate> {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| Coordinate::new(x, y)))
    }
    fn removable(&self) -> impl Iterator<Item = Coordinate> {
        self.coordinates()
            .filter(|coord| matches!(self.get(coord), Some(Tile::Roll)))
            .filter(|coord| {
                let roll_neighbors = self
                    .neighbors(coord)
                    .filter(|(_, tile)| matches!(tile, Tile::Roll))
                    .count();
                roll_neighbors < 4
            })
    }
    fn remove<'a>(&mut self, coordinates: impl Iterator<Item = &'a Coordinate>) {
        coordinates
            .filter(|c| c.x < self.width && c.y < self.height)
            .for_each(|c| self.tiles[c.y][c.x] = Tile::Empty);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                let c = match tile {
                    Tile::Empty => '.',
                    Tile::Roll => '@',
                };
                write!(f, "{} ", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_row(i: &str) -> IResult<&str, Vec<Tile>> {
    terminated(
        many1(alt((
            complete::char('.').map(|_| Tile::Empty),
            complete::char('@').map(|_| Tile::Roll),
        ))),
        opt(complete::newline),
    )
    .parse(i)
}

fn parse_grid(i: &str) -> IResult<&str, Grid> {
    many1(parse_row).map(Grid::new).parse(i)
}

fn parse_input(input: &str) -> Result<Grid> {
    parse_grid(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2025
    }

    fn day(&self) -> Day {
        Day::Day4
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let grid = parse_input(input.trim())?;
        let count = grid.removable().count();
        Ok(count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let mut grid = parse_input(input.trim())?;
        let mut removed = 0;

        loop {
            let removable = grid.removable().collect_vec();
            if removable.is_empty() {
                break;
            }
            removed += removable.len();
            grid.remove(removable.iter());
        }

        Ok(removed.to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "13");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "43");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "1409");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "8366");
        Ok(())
    }
}
