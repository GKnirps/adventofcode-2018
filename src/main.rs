#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();

    let first_line = lines
        .get(0)
        .ok_or("Expected at least one line".to_owned())?;
    let initial_state =
        parse_initial_state(first_line).ok_or("Unable to parse initial state".to_owned())?;
    let rules = parse_rules(&lines[1..]);

    let after_20_gen = run_generations(&initial_state, &rules, 20);
    let sum_after_20_gen = after_20_gen.sum_plant_indices();
    println!(
        "Sum of plant indices after 20 generations: {}",
        sum_after_20_gen
    );

    Ok(())
}

fn run_generations(state: &State, rules: &Rules, n_gen: u32) -> State {
    let mut current_state = state.clone();
    for _ in 0..n_gen {
        current_state = current_state.next_gen(rules);
    }
    return current_state;
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pots: Vec<bool>,
    offset: isize,
}

impl State {
    fn next_gen(&self, rules: &Rules) -> State {
        let mut next_pots: Vec<bool> = Vec::with_capacity(self.pots.len() + 4);
        // right now, we can let the number of pots grow with each generation,
        // we don't expect a large number of them
        let offset = self.offset + 2;
        for i in -2..(self.pots.len() as isize + 2) {
            let neighbours = self.get_neighbours(i);
            next_pots.push(*rules.get(&neighbours).unwrap_or(&neighbours[2]));
        }
        return State {
            pots: next_pots,
            offset,
        };
    }

    fn sum_plant_indices(&self) -> isize {
        self.pots
            .iter()
            .enumerate()
            .filter(|(_, plant)| **plant)
            .map(|(i, _)| i as isize - self.offset)
            .sum()
    }

    fn get_neighbours(&self, index: isize) -> [bool; 5] {
        return [
            self.is_plant(index - 2),
            self.is_plant(index - 1),
            self.is_plant(index),
            self.is_plant(index + 1),
            self.is_plant(index + 2),
        ];
    }
    fn is_plant(&self, index: isize) -> bool {
        if index < 0 {
            return false;
        } else if index >= self.pots.len() as isize {
            return false;
        }
        return self.pots[index as usize];
    }
}

fn parse_initial_state(line: &str) -> Option<State> {
    lazy_static! {
        static ref RE_STATE: Regex = Regex::new(r"initial state: ([.#]+)").unwrap();
    }
    let capture = RE_STATE.captures(line)?;
    let pot_string = capture.get(1)?.as_str();
    let pots: Vec<bool> = pot_string.chars().map(|c| c == '#').collect();
    Some(State { pots, offset: 0 })
}

type Rules = HashMap<[bool; 5], bool>;

fn parse_rules(lines: &[&str]) -> Rules {
    lines.iter().filter_map(|line| parse_rule(line)).collect()
}

fn parse_rule(line: &str) -> Option<([bool; 5], bool)> {
    lazy_static! {
        static ref RE_RULE: Regex = Regex::new(r"([.#]{5}) => ([.#])").unwrap();
    }
    let capture = RE_RULE.captures(line)?;
    let mut con_it = capture.get(1)?.as_str().chars().map(|c| c == '#');
    let condition: [bool; 5] = [
        con_it.next()?,
        con_it.next()?,
        con_it.next()?,
        con_it.next()?,
        con_it.next()?,
    ];
    let result = capture.get(2)?.as_str().chars().next()? == '#';

    return Some((condition, result));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_initial_state_works_correctly() {
        // given
        let line = "initial state: ..###...#foo";

        // when
        let state = parse_initial_state(line).expect("Expected successful parsing");

        // then
        assert_eq!(
            &state.pots,
            &[false, false, true, true, true, false, false, false, true]
        );
        assert_eq!(state.offset, 0);
    }

    #[test]
    fn parse_rule_parses_rule() {
        assert_eq!(
            parse_rule(".#.#. => #"),
            Some(([false, true, false, true, false], true))
        );
        assert_eq!(
            parse_rule("#.#.# => ."),
            Some(([true, false, true, false, true], false))
        );
    }

    #[test]
    fn parse_rules_parses_rules() {
        // given
        let lines = &[".#.#. => #", "#.#.# => ."];

        // when
        let rules = parse_rules(lines);

        // then
        assert_eq!(rules.len(), 2);
        assert_eq!(rules.get(&[false, true, false, true, false]), Some(&true));
    }
}
