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
    let result = execute(&instructions, [0, 0, 0, 0, 0, 0], ip_index)?;
    println!(
        "Puzzle 1: Content of the instructions after the end of the program is {:?}",
        result
    );

    let result_puzzle_2 = execute(&instructions, [1, 0, 0, 0, 0, 0], ip_index)?;
    println!(
        "Puzzle 2: Content of the instructions after the end of the program is {:?}",
        result_puzzle_2
    );

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

    // These tests don't test code here but modified versions of the actual input from day 19
    #[test]
    fn optimized_variants_give_same_results() {
        // given
        let init_reg = [0, 1, 28, 0, 2, 864];

        // when
        let result_base = base_bigloop(init_reg.clone());
        let result_rustified = rustified_bigloop(init_reg.clone());
        let result_rust_opti = optimized_rust_bigloop(init_reg.clone());
        let result_opti = optimized_bigloop(init_reg.clone());

        // then
        println!("Checking elfcode solution");
        assert_eq!(result_base, [2520, 865, 1, 865, 257, 864]);
        println!("Checking rustified solution");
        assert_eq!(result_rustified, result_base);
        println!("Checking optimized rustified solution");
        assert_eq!(result_rust_opti, result_base);
        println!("Checking optimized elfcode solution");
        assert_eq!(result_opti, result_base);
    }

    fn base_bigloop(init_reg: Registers) -> Registers {
        let lines = &[
            "#ip 4",
            "addi 0 0 0",
            "addi 0 0 0",
            "seti 1 7 3",
            "mulr 1 3 2",
            "eqrr 2 5 2",
            "addr 2 4 4",
            "addi 4 1 4",
            "addr 1 0 0",
            "addi 3 1 3",
            "gtrr 3 5 2",
            "addr 4 2 4",
            "seti 2 3 4",
            "addi 1 1 1",
            "gtrr 1 5 2",
            "addr 2 4 4",
            "seti 1 6 4",
            "mulr 4 4 4",
        ];
        let (instructions, ip_index) = parse_program(lines).expect("Expected a valid program.");

        return execute(&instructions, init_reg, ip_index)
            .expect("Expected program to run successfully");
    }

    fn rustified_bigloop(mut r: Registers) -> Registers {
        loop {
            r[3] = 1;
            loop {
                if r[1] * r[3] == r[5] {
                    r[0] += r[1];
                }
                r[3] += 1;
                if r[3] > r[5] {
                    break;
                }
            }
            r[1] += 1;
            if r[1] > r[5] {
                break;
            }
        }
        r[2] = 1;
        r[4] = 257;

        return r;
    }

    fn optimized_rust_bigloop(mut r: Registers) -> Registers {
        for r1 in 1..(r[5] + 1) {
            if r[5] % r1 == 0 {
                r[0] += r1;
            }
        }
        r[1] = r[5] + 1;
        r[2] = 1;
        r[3] = r[5] + 1;
        r[4] = 257;

        return r;
    }

    fn optimized_bigloop(init_reg: Registers) -> Registers {
        let lines = &[
            "#ip 4",
            "addi 0 0 0",
            "addi 0 0 0",
            // loop (10 operations)
            "modr 5 1 2",
            "eqri 2 0 2",
            "addr 2 4 4",
            "addi 4 1 4",
            "addr 1 0 0",
            "addi 5 1 3",
            "seti 1 0 2",
            "addi 0 0 0",
            "addi 0 0 0",
            "addi 0 0 0",
            // loop
            "addi 1 1 1",
            "gtrr 1 5 2",
            "addr 2 4 4",
            "seti 1 6 4",
            "mulr 4 4 4",
        ];
        let (instructions, ip_index) = parse_program(lines).expect("Expected a valid program.");

        return execute(&instructions, init_reg, ip_index)
            .expect("Expected program to run successfully");
    }
}
