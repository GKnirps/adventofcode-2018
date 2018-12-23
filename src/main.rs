#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
    // not part of the original specification, but added for efficiency at some points
    Modr,
}

type Registers = [usize; 6];
type Operands = (usize, usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    operation: OpCode,
    operands: Operands,
}

fn write_into(mut registers: Registers, index: usize, value: usize) -> Option<Registers> {
    if index < registers.len() {
        registers[index] = value;
        return Some(registers);
    }
    return None;
}

fn bool_to_i(b: bool) -> usize {
    if b {
        return 1;
    }
    return 0;
}

fn addr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a)? + *reg.get(op_b)?)
}
fn addi(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a)? + op_b)
}
fn mulr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a)? * *reg.get(op_b)?)
}
fn muli(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a)? * op_b)
}
fn banr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a)? & *reg.get(op_b)?)
}
fn bani(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a)? & op_b)
}
fn borr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a)? | *reg.get(op_b)?)
}
fn bori(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a)? | op_b)
}
fn setr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, _, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a)?)
}
fn seti(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, _, op_c) = *operands;
    write_into(reg, op_c, op_a)
}
fn gtir(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, bool_to_i(op_a > *reg.get(op_b)?))
}
fn gtri(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, bool_to_i(*reg.get(op_a)? > op_b))
}
fn gtrr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, bool_to_i(*reg.get(op_a)? > *reg.get(op_b)?))
}
fn eqir(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, bool_to_i(op_a == *reg.get(op_b)?))
}
fn eqri(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, bool_to_i(*reg.get(op_a)? == op_b))
}
fn eqrr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, bool_to_i(reg.get(op_a)? == reg.get(op_b)?))
}
fn modr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a)? % *reg.get(op_b)?)
}

impl Instruction {
    fn execute(&self, reg: Registers) -> Option<Registers> {
        match self.operation {
            OpCode::Addr => addr(reg, &self.operands),
            OpCode::Addi => addi(reg, &self.operands),
            OpCode::Mulr => mulr(reg, &self.operands),
            OpCode::Muli => muli(reg, &self.operands),
            OpCode::Banr => banr(reg, &self.operands),
            OpCode::Bani => bani(reg, &self.operands),
            OpCode::Borr => borr(reg, &self.operands),
            OpCode::Bori => bori(reg, &self.operands),
            OpCode::Setr => setr(reg, &self.operands),
            OpCode::Seti => seti(reg, &self.operands),
            OpCode::Gtir => gtir(reg, &self.operands),
            OpCode::Gtri => gtri(reg, &self.operands),
            OpCode::Gtrr => gtrr(reg, &self.operands),
            OpCode::Eqir => eqir(reg, &self.operands),
            OpCode::Eqri => eqri(reg, &self.operands),
            OpCode::Eqrr => eqrr(reg, &self.operands),
            OpCode::Modr => modr(reg, &self.operands),
        }
    }
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    let (instructions, ip_index) = parse_program(&lines)?;

    // solution for puzzle 1 (for my input)
    // in my input, exiting the loop was on condition that something in another
    // register equals register 0. So all I had to to was to find out what the value in
    // that other register was at the first time it was reached.
    execute(&instructions, [2985446, 0, 0, 0, 0, 0], ip_index)?;

    Ok(())
}

fn execute(
    instructions: &[Instruction],
    initial_state: Registers,
    ip_index: usize,
) -> Result<Registers, String> {
    let mut state = initial_state;
    if ip_index >= state.len() {
        return Err(format!(
            "Invalid ip index {}, there are only {} registers",
            ip_index,
            state.len()
        ));
    }

    while let Some(instruction) = instructions.get(state[ip_index]) {
        state = instruction
            .execute(state)
            .ok_or_else(|| format!("Unable to execute instruction {:?}", instruction))?;
        state[ip_index] += 1;
    }
    return Ok(state);
}

fn parse_program(lines: &[&str]) -> Result<(Vec<Instruction>, usize), String> {
    lazy_static! {
        static ref RE_IP_INDEX: Regex =
            Regex::new(r"#ip (\d)").expect("Expected instruction pointer index regex to compile");
    }
    if lines.len() == 0 {
        return Err("Cannot parse program: Input is empty.".to_owned());
    }
    let capture = RE_IP_INDEX
        .captures(lines[0])
        .ok_or_else(|| format!("Unable to parse instruction pointer index"))?;
    let ip_index: usize = capture
        .get(1)
        .ok_or_else(|| "Expected match for ip index")?
        .as_str()
        .parse()
        .map_err(|e| format!("instruction pointer index is not a number: {}", e))?;

    let instructions = parse_instructions(&lines[1..])?;

    return Ok((instructions, ip_index));
}

fn parse_instructions(lines: &[&str]) -> Result<Vec<Instruction>, String> {
    lines
        .iter()
        .filter(|l| l.len() != 0)
        .map(|line| {
            parse_instruction(line)
                .ok_or_else(|| format!("instruction line '{}' cannot be parsed", line))
        })
        .collect()
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    lazy_static! {
        static ref RE_OPERATION: Regex =
            Regex::new(r"(\S+) (\d+) (\d+) (\d+)").expect("Expected operation regex to compile");
    }
    let capture = RE_OPERATION.captures(line)?;
    let op_code_str = capture.get(1)?.as_str();
    let op_code: OpCode = match op_code_str {
        "addr" => OpCode::Addr,
        "addi" => OpCode::Addi,
        "mulr" => OpCode::Mulr,
        "muli" => OpCode::Muli,
        "banr" => OpCode::Banr,
        "bani" => OpCode::Bani,
        "borr" => OpCode::Borr,
        "bori" => OpCode::Bori,
        "setr" => OpCode::Setr,
        "seti" => OpCode::Seti,
        "gtir" => OpCode::Gtir,
        "gtri" => OpCode::Gtri,
        "gtrr" => OpCode::Gtrr,
        "eqir" => OpCode::Eqir,
        "eqri" => OpCode::Eqri,
        "eqrr" => OpCode::Eqrr,
        "modr" => OpCode::Modr,
        _ => {
            return None;
        }
    };
    let operands: Operands = (
        capture.get(2)?.as_str().parse().ok()?,
        capture.get(3)?.as_str().parse().ok()?,
        capture.get(4)?.as_str().parse().ok()?,
    );
    return Some(Instruction {
        operation: op_code,
        operands,
    });
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn execute_should_work_for_example() {
        // given
        let lines = &[
            "#ip 0",
            "seti 5 0 1",
            "seti 6 0 2",
            "addi 0 1 0",
            "addr 1 2 3",
            "setr 1 0 0",
            "seti 8 0 4",
            "seti 9 0 5",
        ];
        let (instructions, ip_index) = parse_program(lines).expect("Expected a valid program.");

        // when
        let result = execute(&instructions, [0, 0, 0, 0, 0, 0], ip_index)
            .expect("Expected program to run successfully");

        // then
        assert_eq!(result, [7, 5, 6, 0, 0, 9]);
    }
}
