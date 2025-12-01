use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{preceded, separated_pair, terminated};
use std::cell::RefCell;
use std::collections::HashMap;

struct Circuit {
    wires: HashMap<Wire, Connection>,
    cache: RefCell<HashMap<Wire, u16>>,
}

impl Circuit {
    fn new(connections: &[Connection]) -> Self {
        let wires = connections
            .iter()
            .map(|conn| (conn.output.clone(), conn.clone()))
            .collect();
        let cache = RefCell::new(HashMap::new());
        Self { wires, cache }
    }
    fn override_wire(&mut self, wire: &str, input: ConnInput) {
        let connection = Connection {
            operation: Operation::Signal(input),
            output: wire.to_string(),
        };
        self.wires.insert(wire.to_string(), connection);
        self.cache.borrow_mut().clear();
    }
    fn input_signal(&self, input: &ConnInput) -> Option<u16> {
        match input {
            ConnInput::Raw(value) => Some(*value),
            ConnInput::Wire(wire) => self.wire_signal(wire),
        }
    }
    fn wire_signal(&self, wire: &str) -> Option<u16> {
        if let Some(cached) = self.cache.borrow().get(wire) {
            return Some(*cached);
        }

        let connection = self.wires.get(wire);
        let signal = match connection {
            Some(connection) => match &connection.operation {
                Operation::Signal(value) => self.input_signal(value),
                Operation::And(left, right) => {
                    let left_signal = self.input_signal(left)?;
                    let right_signal = self.input_signal(right)?;
                    Some(left_signal & right_signal)
                }
                Operation::Or(left, right) => {
                    let left_signal = self.input_signal(left)?;
                    let right_signal = self.input_signal(right)?;
                    Some(left_signal | right_signal)
                }
                Operation::LShift(input, shift) => {
                    let input_signal = self.input_signal(input)?;
                    Some(input_signal << shift)
                }
                Operation::RShift(input, shift) => {
                    let input_signal = self.input_signal(input)?;
                    Some(input_signal >> shift)
                }
                Operation::Not(input) => {
                    let input_signal = self.input_signal(input)?;
                    Some(!input_signal)
                }
            },
            None => None,
        };

        if let Some(sig) = signal {
            self.cache.borrow_mut().insert(wire.to_string(), sig);
        }

        signal
    }
}

type Wire = String;

#[derive(Debug, Clone)]
enum ConnInput {
    Raw(u16),
    Wire(Wire),
}

#[derive(Debug, Clone)]
enum Operation {
    Signal(ConnInput),
    And(ConnInput, ConnInput),
    Or(ConnInput, ConnInput),
    LShift(ConnInput, u16),
    RShift(ConnInput, u16),
    Not(ConnInput),
}

#[derive(Debug, Clone)]
struct Connection {
    operation: Operation,
    output: Wire,
}

fn parse_wire(input: &str) -> IResult<&str, Wire> {
    complete::alpha1.map(|s: &str| s.to_string()).parse(input)
}

fn parse_connection_input(i: &str) -> IResult<&str, ConnInput> {
    alt((
        complete::u16.map(ConnInput::Raw),
        parse_wire.map(ConnInput::Wire),
    ))
    .parse(i)
}

fn parse_signal_connection(i: &str) -> IResult<&str, Connection> {
    map(
        separated_pair(parse_connection_input, tag(" -> "), parse_wire),
        |(input, output)| Connection {
            operation: Operation::Signal(input),
            output,
        },
    )
    .parse(i)
}

fn parse_and_connection(i: &str) -> IResult<&str, Connection> {
    map(
        separated_pair(
            separated_pair(parse_connection_input, tag(" AND "), parse_connection_input),
            tag(" -> "),
            parse_wire,
        ),
        |((left, right), output)| Connection {
            operation: Operation::And(left, right),
            output,
        },
    )
    .parse(i)
}

fn parse_or_connection(i: &str) -> IResult<&str, Connection> {
    map(
        separated_pair(
            separated_pair(parse_connection_input, tag(" OR "), parse_connection_input),
            tag(" -> "),
            parse_wire,
        ),
        |((left, right), output)| Connection {
            operation: Operation::Or(left, right),
            output,
        },
    )
    .parse(i)
}

fn parse_not_connection(i: &str) -> IResult<&str, Connection> {
    map(
        separated_pair(
            preceded(tag("NOT "), parse_connection_input),
            tag(" -> "),
            parse_wire,
        ),
        |(input, output)| Connection {
            operation: Operation::Not(input),
            output,
        },
    )
    .parse(i)
}

fn parse_lshift_connection(i: &str) -> IResult<&str, Connection> {
    map(
        separated_pair(
            separated_pair(parse_connection_input, tag(" LSHIFT "), complete::u16),
            tag(" -> "),
            parse_wire,
        ),
        |((input, shift), output)| Connection {
            operation: Operation::LShift(input, shift),
            output,
        },
    )
    .parse(i)
}

fn parse_rshift_connection(i: &str) -> IResult<&str, Connection> {
    map(
        separated_pair(
            separated_pair(parse_connection_input, tag(" RSHIFT "), complete::u16),
            tag(" -> "),
            parse_wire,
        ),
        |((input, shift), output)| Connection {
            operation: Operation::RShift(input, shift),
            output,
        },
    )
    .parse(i)
}

fn parse_connection(i: &str) -> IResult<&str, Connection> {
    terminated(
        alt((
            parse_signal_connection,
            parse_and_connection,
            parse_or_connection,
            parse_not_connection,
            parse_lshift_connection,
            parse_rshift_connection,
        )),
        opt(complete::newline),
    )
    .parse(i)
}

fn parse_circuit(i: &str) -> IResult<&str, Circuit> {
    let (i, connections) = many1(parse_connection).parse(i)?;
    let circuit = Circuit::new(&connections);
    Ok((i, circuit))
}

fn parse_input(i: &str) -> Result<Circuit> {
    parse_circuit(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2015
    }

    fn day(&self) -> Day {
        Day::Day7
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let circuit = parse_input(input)?;
        let signal = circuit.wire_signal("a").ok_or_else(|| {
            AdventError::Other("Could not determine signal for wire 'a'".to_string())
        })?;
        Ok(signal.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let mut circuit = parse_input(input)?;
        let signal = circuit.wire_signal("a").ok_or_else(|| {
            AdventError::Other("Could not determine signal for wire 'a'".to_string())
        })?;
        circuit.override_wire("b", ConnInput::Raw(signal));
        let new_signal = circuit.wire_signal("a").ok_or_else(|| {
            AdventError::Other("Could not determine signal for wire 'a' after override".to_string())
        })?;
        Ok(new_signal.to_string())
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
        let circuit = parse_input(&input)?;

        assert_eq!(circuit.wire_signal("a"), None);

        assert_eq!(circuit.wire_signal("d"), Some(72));
        assert_eq!(circuit.wire_signal("e"), Some(507));
        assert_eq!(circuit.wire_signal("f"), Some(492));
        assert_eq!(circuit.wire_signal("g"), Some(114));
        assert_eq!(circuit.wire_signal("h"), Some(65412));
        assert_eq!(circuit.wire_signal("i"), Some(65079));
        assert_eq!(circuit.wire_signal("x"), Some(123));
        assert_eq!(circuit.wire_signal("y"), Some(456));

        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "16076");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "2797");
        Ok(())
    }
}
