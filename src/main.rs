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
}

type Registers = [u32; 4];
type Operands = (u32, u32, u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    operation: OpCode,
    operands: Operands,
}

fn write_into(mut registers: Registers, index: u32, value: u32) -> Option<Registers> {
    if index < 4 {
        registers[index as usize] = value;
        return Some(registers);
    }
    return None;
}

fn bool_to_i(b: bool) -> u32 {
    if b {
        return 1;
    }
    return 0;
}

fn addr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(
        reg,
        op_c,
        *reg.get(op_a as usize)? + *reg.get(op_b as usize)?,
    )
}
fn addi(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a as usize)? + op_b)
}
fn mulr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(
        reg,
        op_c,
        *reg.get(op_a as usize)? * *reg.get(op_b as usize)?,
    )
}
fn muli(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a as usize)? * op_b)
}
fn banr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(
        reg,
        op_c,
        *reg.get(op_a as usize)? & *reg.get(op_b as usize)?,
    )
}
fn bani(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a as usize)? & op_b)
}
fn borr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(
        reg,
        op_c,
        *reg.get(op_a as usize)? | *reg.get(op_b as usize)?,
    )
}
fn bori(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a as usize)? | op_b)
}
fn setr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, _, op_c) = *operands;
    write_into(reg, op_c, *reg.get(op_a as usize)?)
}
fn seti(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, _, op_c) = *operands;
    write_into(reg, op_c, op_a)
}
fn gtir(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, bool_to_i(op_a > *reg.get(op_b as usize)?))
}
fn gtri(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, bool_to_i(*reg.get(op_a as usize)? > op_b))
}
fn gtrr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(
        reg,
        op_c,
        bool_to_i(*reg.get(op_a as usize)? > *reg.get(op_b as usize)?),
    )
}
fn eqir(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, bool_to_i(op_a == *reg.get(op_b as usize)?))
}
fn eqri(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(reg, op_c, bool_to_i(*reg.get(op_a as usize)? == op_b))
}
fn eqrr(reg: Registers, operands: &Operands) -> Option<Registers> {
    let (op_a, op_b, op_c) = *operands;
    write_into(
        reg,
        op_c,
        bool_to_i(reg.get(op_a as usize)? == reg.get(op_b as usize)?),
    )
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
        }
    }
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let sections: Vec<&str> = content.split("\n\n\n\n").collect();
    if sections.len() != 2 {
        return Err(format!(
            "Expected two sections in the input, found {}",
            sections.len()
        ));
    }

    let observation_blocks: Vec<&str> = sections[0].split("\n\n").collect();
    let observations = parse_observations(&observation_blocks)?;
    let ambig_count = samples_with_more_than_three_possible_ops(&observations);
    println!(
        "There are {} samples with more than three possible operations (out of {})",
        ambig_count,
        observations.len()
    );

    Ok(())
}

fn count_ambiguous_observations(observation: &Observation) -> u32 {
    static OP_CODES: [OpCode; 16] = [
        OpCode::Addr,
        OpCode::Addi,
        OpCode::Mulr,
        OpCode::Muli,
        OpCode::Banr,
        OpCode::Bani,
        OpCode::Borr,
        OpCode::Bori,
        OpCode::Setr,
        OpCode::Seti,
        OpCode::Gtir,
        OpCode::Gtri,
        OpCode::Gtrr,
        OpCode::Eqir,
        OpCode::Eqri,
        OpCode::Eqrr,
    ];
    OP_CODES
        .iter()
        .map(|oc| Instruction {
            operation: *oc,
            operands: observation.operands,
        })
        .filter(|op| op.execute(observation.before.clone()) == Some(observation.after))
        .count() as u32
}

fn samples_with_more_than_three_possible_ops(samples: &[Observation]) -> usize {
    samples
        .iter()
        .map(|obs| count_ambiguous_observations(obs))
        .filter(|c| *c >= 3)
        .count()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Observation {
    before: Registers,
    op_id: u32,
    operands: Operands,
    after: Registers,
}

fn parse_operation_line(line: &str) -> Option<(u32, Operands)> {
    lazy_static! {
        static ref RE_OPERATION: Regex =
            Regex::new(r"(\d+) (\d+) (\d+) (\d+)").expect("Expected operation regex to compile");
    }
    let capture = RE_OPERATION.captures(line)?;
    let op_id: u32 = capture.get(1)?.as_str().parse().ok()?;
    let operands: Operands = (
        capture.get(2)?.as_str().parse().ok()?,
        capture.get(3)?.as_str().parse().ok()?,
        capture.get(4)?.as_str().parse().ok()?,
    );
    return Some((op_id, operands));
}

fn parse_observations(blocks: &[&str]) -> Result<Vec<Observation>, String> {
    blocks
        .iter()
        .map(|b| parse_observation(b))
        .collect::<Option<Vec<Observation>>>()
        .ok_or_else(|| "Unable to parse all observations".to_owned())
}

fn parse_observation(block: &str) -> Option<Observation> {
    lazy_static! {
        static ref RE_BEFORE: Regex = Regex::new(r"Before: *\[(\d), (\d), (\d), (\d)\]")
            .expect("Expected before-register-regex to parse");
        static ref RE_AFTER: Regex = Regex::new(r"After: *\[(\d), (\d), (\d), (\d)\]")
            .expect("Expected after-register-regex to parse");
    }
    let lines: Vec<&str> = block.split('\n').collect();
    if lines.len() != 3 {
        return None;
    }
    let b_capture = RE_BEFORE.captures(lines[0])?;
    let before: Registers = [
        b_capture.get(1)?.as_str().parse().ok()?,
        b_capture.get(2)?.as_str().parse().ok()?,
        b_capture.get(3)?.as_str().parse().ok()?,
        b_capture.get(4)?.as_str().parse().ok()?,
    ];
    let (op_id, operands) = parse_operation_line(lines[1])?;
    let a_capture = RE_AFTER.captures(lines[2])?;
    let after: Registers = [
        a_capture.get(1)?.as_str().parse().ok()?,
        a_capture.get(2)?.as_str().parse().ok()?,
        a_capture.get(3)?.as_str().parse().ok()?,
        a_capture.get(4)?.as_str().parse().ok()?,
    ];
    return Some(Observation {
        before,
        op_id,
        operands,
        after,
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
    fn observation_parsing_works_correctly() {
        // given
        let block = "Before: [0, 1, 2, 3]\n10 20 30 40\nAfter: [4, 5, 6, 7]";

        // when
        let observation = parse_observation(block).expect("Expected a valid observation");

        // then
        assert_eq!(observation.before, [0, 1, 2, 3]);
        assert_eq!(observation.op_id, 10);
        assert_eq!(observation.operands, (20, 30, 40));
        assert_eq!(observation.after, [4, 5, 6, 7]);
    }

    #[test]
    fn observation_parsing_works_for_actual_input() {
        // given
        let block = "Before: [1, 0, 1, 3]\n9 2 1 0\nAfter:  [2, 0, 1, 3]";

        // when
        let observation = parse_observation(block).expect("Expected a valid observation");

        // then
        assert_eq!(observation.before, [1, 0, 1, 3]);
        assert_eq!(observation.op_id, 9);
        assert_eq!(observation.operands, (2, 1, 0));
        assert_eq!(observation.after, [2, 0, 1, 3]);
    }
}
