use std::collections::HashSet;
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
    None
}

fn bool_to_i(b: bool) -> u32 {
    if b {
        return 1;
    }
    0
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
    let content = read_file(Path::new(&filename)).map_err(|e| e.to_string())?;
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

    let op_codes = op_code_map(&observations)?;
    let instruction_lines: Vec<&str> = sections[1].split('\n').collect();
    let instructions = parse_instructions(&instruction_lines, &op_codes)?;
    let result = execute(&instructions, [0, 0, 0, 0])?;
    println!("Result registers: {:?}", result);

    Ok(())
}

fn execute(instructions: &[Instruction], initial_state: Registers) -> Result<Registers, String> {
    let mut state = initial_state;
    for instruction in instructions {
        state = instruction
            .execute(state)
            .ok_or_else(|| format!("Unable to execute instruction {:?}", instruction))?;
    }
    Ok(state)
}

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
fn possible_op_codes(observation: &Observation) -> HashSet<OpCode> {
    OP_CODES
        .iter()
        .copied()
        .filter(|oc| {
            Instruction {
                operation: *oc,
                operands: observation.operands,
            }
            .execute(observation.before)
                == Some(observation.after)
        })
        .collect()
}

fn op_code_map(samples: &[Observation]) -> Result<Vec<OpCode>, String> {
    let all_op_codes: HashSet<OpCode> = OP_CODES.iter().cloned().collect();
    let mut map: Vec<HashSet<OpCode>> = (0..OP_CODES.len()).map(|_| all_op_codes.clone()).collect();
    for sample in samples {
        if sample.op_id >= map.len() as u32 {
            return Err(format!(
                "Error matching op codes: {} is not a valid op code (must be < {})",
                sample.op_id,
                map.len()
            ));
        }
        let op_codes = possible_op_codes(sample);
        map[sample.op_id as usize] = map[sample.op_id as usize]
            .intersection(&op_codes)
            .cloned()
            .collect();
    }
    let mut non_ambig: HashSet<OpCode> = map
        .iter()
        .filter_map(|ocs| {
            if ocs.len() != 1 {
                return None;
            }
            return ocs.iter().next().copied();
        })
        .collect();
    let mut prev_non_ambig_count = 0;
    while prev_non_ambig_count != non_ambig.len() {
        map = map
            .into_iter()
            .map(|s| {
                if s.len() == 1 {
                    return s;
                }
                return s.difference(&non_ambig).cloned().collect();
            })
            .collect();
        prev_non_ambig_count = non_ambig.len();
        non_ambig = map
            .iter()
            .filter_map(|ocs| {
                if ocs.len() != 1 {
                    return None;
                }
                return ocs.iter().next().copied();
            })
            .collect();
    }
    return map
        .iter()
        .map(|ocs| {
            if ocs.len() != 1 {
                return None;
            }
            return ocs.iter().next().copied();
        })
        .collect::<Option<Vec<OpCode>>>()
        .ok_or_else(|| "Error matching op codes: There are still unknown op codes".to_owned());
}

fn samples_with_more_than_three_possible_ops(samples: &[Observation]) -> usize {
    samples
        .iter()
        .map(possible_op_codes)
        .filter(|codes| codes.len() >= 3)
        .count()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Observation {
    before: Registers,
    op_id: u32,
    operands: Operands,
    after: Registers,
}

fn parse_instructions(lines: &[&str], op_codes: &[OpCode]) -> Result<Vec<Instruction>, String> {
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (op_id, operands) = parse_operation_line(line)
                .ok_or_else(|| format!("instruction line '{}' cannot be parsed", line))?;
            let op_code = op_codes
                .get(op_id as usize)
                .ok_or_else(|| format!("Unknown op code: {}", op_id))?;
            Ok(Instruction {
                operation: *op_code,
                operands,
            })
        })
        .collect()
}

fn parse_operation_line(line: &str) -> Option<(u32, Operands)> {
    let mut split = line.splitn(4, ' ');
    let op_id: u32 = split.next()?.parse().ok()?;
    let operands: Operands = (
        split.next()?.parse().ok()?,
        split.next()?.parse().ok()?,
        split.next()?.parse().ok()?,
    );
    Some((op_id, operands))
}

fn parse_observations(blocks: &[&str]) -> Result<Vec<Observation>, String> {
    blocks
        .iter()
        .map(|b| parse_observation(b))
        .collect::<Option<Vec<Observation>>>()
        .ok_or_else(|| "Unable to parse all observations".to_owned())
}

fn parse_observation(block: &str) -> Option<Observation> {
    let mut lines = block.lines();
    let mut befores = lines
        .next()?
        .strip_prefix("Before: [")?
        .strip_suffix(']')?
        .splitn(4, ", ");
    let before: Registers = [
        befores.next()?.parse().ok()?,
        befores.next()?.parse().ok()?,
        befores.next()?.parse().ok()?,
        befores.next()?.parse().ok()?,
    ];
    let (op_id, operands) = parse_operation_line(lines.next()?)?;
    let mut afters = lines
        .next()?
        .strip_prefix("After:  [")?
        .strip_suffix(']')?
        .splitn(4, ", ");
    let after: Registers = [
        afters.next()?.parse().ok()?,
        afters.next()?.parse().ok()?,
        afters.next()?.parse().ok()?,
        afters.next()?.parse().ok()?,
    ];
    if lines.next().is_some() {
        return None;
    }
    Some(Observation {
        before,
        op_id,
        operands,
        after,
    })
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn observation_parsing_works_correctly() {
        // given
        let block = "Before: [0, 1, 2, 3]\n10 20 30 40\nAfter:  [4, 5, 6, 7]";

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
