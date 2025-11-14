use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use std::ops::{BitXorAssign, Deref};
use z3::ast::{Ast, BV};

struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl Registers {
    fn new(a: u64, b: u64, c: u64) -> Self {
        Self { a, b, c }
    }
}

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn operand(&self, value: u64) -> Operand {
        match self {
            Instruction::Adv => Operand::Combo(value),
            Instruction::Bxl => Operand::Literal(value),
            Instruction::Bst => Operand::Combo(value),
            Instruction::Jnz => Operand::Literal(value),
            Instruction::Bxc => Operand::Ignored(value),
            Instruction::Out => Operand::Combo(value),
            Instruction::Bdv => Operand::Combo(value),
            Instruction::Cdv => Operand::Combo(value),
        }
    }
}

enum Operand {
    Literal(u64),
    Combo(u64),
    Ignored(u64),
}

impl Operand {
    fn value(&self, registers: &Registers) -> u64 {
        match self {
            Operand::Literal(c) => *c,
            Operand::Combo(c) => match *c {
                0..=3 => *c,
                4 => registers.a,
                5 => registers.b,
                6 => registers.c,
                7 => panic!("Reserved value"),
                _ => panic!("Invalid value"),
            },
            Operand::Ignored(c) => *c,
        }
    }
}

impl TryFrom<u64> for Instruction {
    type Error = ();

    fn try_from(value: u64) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Instruction::Adv),
            1 => Ok(Instruction::Bxl),
            2 => Ok(Instruction::Bst),
            3 => Ok(Instruction::Jnz),
            4 => Ok(Instruction::Bxc),
            5 => Ok(Instruction::Out),
            6 => Ok(Instruction::Bdv),
            7 => Ok(Instruction::Cdv),
            _ => Err(()),
        }
    }
}

type Program = Vec<u64>;

trait Executable {
    fn execute(&self, registers: &mut Registers) -> Vec<u64>;
}

trait Reversible {
    fn reverse(&self, registers: &mut Registers) -> u64;
}

struct FlexiExecutable {
    program: Program,
}
struct SmallExecutable1 {}
struct SmallExecutable2 {}
struct LargeExecutable {}

impl Reversible for SmallExecutable2 {
    fn reverse(&self, registers: &mut Registers) -> u64 {
        // The small executable is:
        // 0,3,5,4,3,0 which translates to
        //
        // a = a >> 3
        // out a
        // jnz -
        //
        // A must be zero at the end.

        let program = vec![0, 3, 5, 4, 3, 0];
        let mut register = 0;

        // build up a value matching (only possible because of how the program executes)
        //      0   3   4   5   3   0   0
        // 0b_000_011_100_101_011_000_000

        for val in program.into_iter().rev() {
            register |= val & 0b111;
            register <<= 3;
        }
        register
    }
}
impl Reversible for LargeExecutable {
    fn reverse(&self, registers: &mut Registers) -> u64 {
        // The large executable is:
        // 2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0 which translates to
        //
        // b = a % 8
        // b = b ^ 5
        // c = a >> b
        // b = b ^ 6
        // b = b ^ c
        // a = a >> 3
        // out b
        // jnz -
        //
        // A must be zero at the end.

        let program = vec![2, 4, 1, 5, 7, 5, 4, 3, 1, 6, 0, 3, 5, 5, 3, 0];
        let mut register = 0;

        let cfg = z3::Config::new();
        let opt = z3::Optimize::new();

        let s = BV::new_const("s", 64);

        let mut a = s.clone();
        let mut b = BV::from_u64(0, 64);
        let mut c = BV::from_u64(0, 64);

        for x in program {
            // Execute the program.
            b = &a & BV::from_u64(7, 64);
            b ^= BV::from_u64(5, 64);
            c = a.bvlshr(&b);
            b ^= BV::from_u64(6, 64);
            b ^= c;
            a = a.bvlshr(&BV::from_u64(3, 64));
            // Assert that B's lower bits outputs the current program value.
            opt.assert(&(&b & &BV::from_i64(7, 64)).eq(&BV::from_i64(x, 64)));
        }

        // Assert that A is zero for program termination.
        opt.assert(&(a.eq(&BV::from_i64(0, 64))));

        // Look to keeping the register value as low as possible.
        opt.minimize(&s);

        match opt.check(&[]) {
            z3::SatResult::Sat => opt
                .get_model()
                .expect("Model")
                .eval(&s, true)
                .expect("Eval")
                .as_u64()
                .expect("U64"),
            _ => panic!("Unsat"),
        }
    }
}

impl Executable for FlexiExecutable {
    fn execute(&self, registers: &mut Registers) -> Vec<u64> {
        let mut output = vec![];
        let mut ip = 0;
        while ip < self.program.len() {
            let instruction = Instruction::try_from(self.program[ip]).expect("Invalid instruction");
            let operand = instruction.operand(self.program[ip + 1]).value(registers);
            ip += 2;

            match instruction {
                Instruction::Adv => {
                    registers.a >>= operand;
                }
                Instruction::Bxl => {
                    registers.b.bitxor_assign(operand);
                }
                Instruction::Bst => {
                    registers.b = operand % 8;
                }
                Instruction::Jnz => {
                    if registers.a != 0 {
                        ip = operand as usize;
                    }
                }
                Instruction::Bxc => registers.b.bitxor_assign(registers.c),
                Instruction::Out => {
                    output.push(operand % 8);
                }
                Instruction::Bdv => {
                    registers.b = registers.a >> operand;
                }
                Instruction::Cdv => {
                    registers.c = registers.a >> operand;
                }
            }
        }
        output
    }
}

impl Executable for SmallExecutable1 {
    fn execute(&self, registers: &mut Registers) -> Vec<u64> {
        let mut output = vec![];
        loop {
            registers.a >>= 1;
            output.push(registers.a % 8);
            if registers.a == 0 {
                break;
            }
        }
        output
    }
}

impl Executable for LargeExecutable {
    fn execute(&self, registers: &mut Registers) -> Vec<u64> {
        let mut output = vec![];
        loop {
            registers.b = registers.a % 8;
            registers.b.bitxor_assign(5);
            registers.c = registers.a >> registers.b;
            registers.b.bitxor_assign(6);
            registers.b.bitxor_assign(registers.c);
            registers.a >>= 3;
            output.push(registers.b % 8);
            if registers.a == 0 {
                break;
            }
        }
        output
    }
}

struct Computer {
    registers: Registers,
}

impl Computer {
    fn new(registers: Registers) -> Self {
        Self { registers }
    }
    fn run(&mut self, executable: &dyn Executable) -> Vec<u64> {
        executable.execute(&mut self.registers)
    }
    fn reverse(&mut self, executable: &dyn Reversible) -> u64 {
        executable.reverse(&mut self.registers)
    }
}

fn parse_register_value<'a>(i: &'a str, register: &'a str) -> IResult<&'a str, u64> {
    let (i, _) = tag("Register ").parse(i)?;
    let (i, _) = tag(register).parse(i)?;
    let (i, _) = tag(": ").parse(i)?;
    let (i, value) = complete::u64(i)?;
    let (i, _) = newline(i)?;
    Ok((i, value))
}

fn parse_program(i: &str) -> IResult<&str, Program> {
    let (i, _) = tag("Program: ").parse(i)?;
    let (i, program) = separated_list1(tag(","), complete::u64).parse(i)?;
    Ok((i, program))
}

fn parse_computer(i: &str) -> IResult<&str, (Computer, Program)> {
    let (i, a) = parse_register_value(i, "A")?;
    let (i, b) = parse_register_value(i, "B")?;
    let (i, c) = parse_register_value(i, "C")?;
    let (i, _) = newline(i)?;
    let (i, program) = parse_program(i)?;
    let registers = Registers::new(a, b, c);
    let computer = Computer::new(registers);
    Ok((i, (computer, program)))
}

fn parse_input(input: &str) -> Result<(Computer, Program)> {
    parse_computer(input).map_and_finish()
}

enum SolverKind {
    Flexi,
    Small1,
    Small2,
    Large,
}

struct Solver {
    kind: SolverKind,
}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day17
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let (mut computer, program) = parse_input(input)?;
        let executable: Box<dyn Executable> = match self.kind {
            SolverKind::Flexi => Box::new(FlexiExecutable { program }),
            SolverKind::Small1 => Box::new(SmallExecutable1 {}),
            SolverKind::Small2 => return Err(AdventError::Other("Unsupported kind".to_string())),
            SolverKind::Large => Box::new(LargeExecutable {}),
        };
        let output = computer.run(executable.deref());
        Ok(output.iter().map(|c| c.to_string()).collect_vec().join(","))
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let (mut computer, _) = parse_input(input)?;
        let executable: Box<dyn Reversible> = match self.kind {
            SolverKind::Flexi | SolverKind::Small1 => {
                return Err(AdventError::Other("Unsupported kind".to_string()));
            }
            SolverKind::Small2 => Box::new(SmallExecutable2 {}),
            SolverKind::Large => Box::new(LargeExecutable {}),
        };
        let register_a = computer.reverse(executable.deref());
        Ok(register_a.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::flexi(SolverKind::Flexi)]
    #[case::small(SolverKind::Small1)]
    fn example1(#[case] kind: SolverKind) -> Result<()> {
        let solver = Solver { kind };
        let input = solver.read_resource(Input::Example1)?;
        assert_eq!(solver.solve_part1(&input)?, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[rstest]
    #[case::small(SolverKind::Small2)]
    fn example2(#[case] kind: SolverKind) -> Result<()> {
        let solver = Solver { kind };
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "117440");
        Ok(())
    }

    #[rstest]
    #[case::flexi(SolverKind::Flexi)]
    #[case::small(SolverKind::Large)]
    fn part1(#[case] kind: SolverKind) -> Result<()> {
        let solver = Solver { kind };
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "7,3,5,7,5,7,4,3,0");
        Ok(())
    }

    #[rstest]
    #[case::small(SolverKind::Large)]
    fn part2(#[case] kind: SolverKind) -> Result<()> {
        let solver = Solver { kind };
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "105734774294938");
        Ok(())
    }
}
