#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();

    Ok(())
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

struct State {
    pots: VecDeque<bool>,
    offset_left: usize,
}

fn parse_initial_state(line: &str) -> Option<State> {
    lazy_static! {
        static ref RE_STATE: Regex = Regex::new(r"initial state: ([.#]+)").unwrap();
    }
    let capture = RE_STATE.captures(line)?;
    let pot_string = capture.get(1)?.as_str();
    let pots: VecDeque<bool> = pot_string.chars().map(|c| c == '#').collect();
    Some(State {
        pots,
        offset_left: 0,
    })
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
        assert_eq!(state.offset_left, 0);
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
