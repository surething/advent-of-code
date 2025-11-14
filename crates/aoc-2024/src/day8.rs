use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::AsChar;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, satisfy};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use std::collections::{HashMap, HashSet};
use std::ops::Range;

type Frequency = char;

enum Tile {
    Empty,
    Antenna(Frequency),
    Antinode,
}

struct Antenna {
    freq: char,
    coordinate: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    row: i32,
    col: i32,
}

type Coordinates = HashSet<Coordinate>;

impl Coordinate {
    fn antinode_for(&self, other: &Self, n: usize) -> Self {
        let delta_row = self.row - other.row;
        let delta_col = self.col - other.col;
        let row = self.row + delta_row * n as i32;
        let col = self.col + delta_col * n as i32;
        Self { row, col }
    }
    fn antinodes_within(&self, other: &Self, rows: &Range<i32>, cols: &Range<i32>) -> Vec<Self> {
        let delta_row = self.row - other.row;
        let delta_col = self.col - other.col;
        if delta_row == 0 && delta_col == 0 {
            return vec![*self];
        }
        let mut antinodes = vec![];
        for n in 0.. {
            let antinode = self.antinode_for(other, n);
            if rows.contains(&antinode.row) && cols.contains(&antinode.col) {
                antinodes.push(antinode);
            } else {
                break;
            }
        }
        antinodes
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    antennas: Vec<Antenna>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let rows = tiles.len();
        let cols = tiles.first().map_or(0, |row| row.len());
        assert!(tiles.iter().all(|row| row.len() == cols));
        let antennas = tiles
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, tile)| match tile {
                        Tile::Antenna(freq) => Some(Antenna {
                            freq: *freq,
                            coordinate: Coordinate {
                                row: i as i32,
                                col: j as i32,
                            },
                        }),
                        _ => None,
                    })
            })
            .collect();
        Self {
            tiles,
            antennas,
            rows,
            cols,
        }
    }
    fn show(&self) {
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Empty => print!("."),
                    Tile::Antenna(f) => print!("{}", f),
                    Tile::Antinode => print!("#"),
                }
                print!(" ");
            }
            println!();
        }
    }
    fn show_with_antinodes(&self) {
        let antinodes = self
            .coordinates_by_frequency()
            .iter()
            .flat_map(|(_freq, coord)| {
                let rows = 0..self.rows as i32;
                let cols = 0..self.cols as i32;
                coord
                    .iter()
                    .tuple_combinations::<(_, _)>()
                    .flat_map(move |(c1, c2)| {
                        let a1 = c1.antinodes_within(c2, &rows, &cols);
                        let a2 = c2.antinodes_within(c1, &rows, &cols);
                        a1.iter().chain(a2.iter()).cloned().collect_vec()
                    })
            })
            .collect::<HashSet<_>>();

        for (i, row) in self.tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                let coord = Coordinate {
                    row: i as i32,
                    col: j as i32,
                };
                match tile {
                    Tile::Empty => {
                        if antinodes.contains(&coord) {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    Tile::Antenna(f) => print!("{}", f),
                    Tile::Antinode => print!("#"),
                }
                print!(" ");
            }
            println!();
        }
    }
    fn is_inside(&self, coord: &Coordinate) -> bool {
        coord.row >= 0
            && coord.row < self.rows as i32
            && coord.col >= 0
            && coord.col < self.cols as i32
    }
    fn coordinates_by_frequency(&self) -> HashMap<Frequency, Coordinates> {
        self.antennas
            .iter()
            .fold(HashMap::new(), |mut acc, antenna| {
                let freq = antenna.freq;
                let coord = antenna.coordinate;
                let entry = acc.entry(freq).or_default();
                entry.insert(coord);
                acc
            })
    }
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, tiles) = separated_list1(
        newline,
        many1(alt((
            map(tag("."), |_| Tile::Empty),
            map(satisfy(|c| !AsChar::is_newline(c as u8)), Tile::Antenna),
        ))),
    )
    .parse(input)?;
    let map = Map::new(tiles);
    Ok((input, map))
}

fn parse_input(input: &str) -> Result<Map> {
    parse_map(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day8
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let map = parse_input(input)?;
        // map.show();
        let result = map
            .coordinates_by_frequency()
            .iter()
            .flat_map(|(_freq, coord)| {
                coord
                    .iter()
                    .tuple_combinations::<(_, _)>()
                    .flat_map(|(c1, c2)| {
                        let a1 = c1.antinode_for(c2, 1);
                        let a2 = c2.antinode_for(c1, 1);
                        vec![a1, a2]
                    })
            })
            .filter(|c| map.is_inside(c))
            .collect::<HashSet<_>>()
            .len();
        Ok(result.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let map = parse_input(input)?;
        let result = map
            .coordinates_by_frequency()
            .iter()
            .flat_map(|(_freq, coord)| {
                let rows = 0..map.rows as i32;
                let cols = 0..map.cols as i32;
                coord
                    .iter()
                    .tuple_combinations::<(_, _)>()
                    .flat_map(move |(c1, c2)| {
                        let a1 = c1.antinodes_within(c2, &rows, &cols);
                        let a2 = c2.antinodes_within(c1, &rows, &cols);
                        a1.iter().chain(a2.iter()).cloned().collect_vec()
                    })
            })
            .collect::<HashSet<_>>()
            .len();
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
        assert_eq!(solver.solve_part1(&input)?, "14");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "34");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "392");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "1235");
        Ok(())
    }

    #[rstest]
    fn antinode() -> Result<()> {
        let c1 = Coordinate { row: 3, col: 4 };
        let c2 = Coordinate { row: 5, col: 5 };

        let a = c1.antinode_for(&c2, 1);
        assert_eq!(a, Coordinate { row: 1, col: 3 });
        let a = c1.antinode_for(&c2, 2);
        assert_eq!(a, Coordinate { row: -1, col: 2 });
        let a = c1.antinode_for(&c2, 3);
        assert_eq!(a, Coordinate { row: -3, col: 1 });

        let a = c2.antinode_for(&c1, 1);
        assert_eq!(a, Coordinate { row: 7, col: 6 });
        let a = c2.antinode_for(&c1, 2);
        assert_eq!(a, Coordinate { row: 9, col: 7 });
        let a = c2.antinode_for(&c1, 3);
        assert_eq!(a, Coordinate { row: 11, col: 8 });

        Ok(())
    }
}
