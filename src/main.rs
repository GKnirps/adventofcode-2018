use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let trimmed = content.trim();

    let reacted = react_polymer(trimmed, None);

    println!(
        "After reacting, the polymer has a length of {} units",
        reacted.chars().count()
    );

    let units = available_units(trimmed);

    let shortest = units
        .iter()
        .map(|unit| react_polymer(trimmed, Some(*unit)).chars().count())
        .min();
    if let Some(length) = shortest {
        println!(
            "After removing one unit type, the shortest reacted polymer has a length of {}.",
            length
        );
    }

    Ok(())
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

fn available_units(pol: &str) -> HashSet<char> {
    pol.chars().map(|c| c.to_ascii_lowercase()).collect()
}

fn react_polymer(pol: &str, filter_element: Option<char>) -> String {
    let mut stack: Vec<char> = Vec::with_capacity(pol.len());
    for c in pol.chars().filter(|c| {
        filter_element
            .map(|f| c.to_ascii_lowercase() != f)
            .unwrap_or(true)
    }) {
        if chars_match(stack.last().map(|c| c.clone()), c) {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    return stack.iter().collect();
}

fn chars_match(opt_left: Option<char>, right: char) -> bool {
    if let Some(left) = opt_left {
        return left.to_ascii_lowercase() == right.to_ascii_lowercase()
            && (left.is_uppercase() && right.is_lowercase()
                || left.is_lowercase() && right.is_uppercase());
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn react_polymer_works_correctly() {
        // given
        let input = "dabAcCaCBAcCcaDA";

        // when
        let result = react_polymer(input, None);

        // then
        assert_eq!(&result, "dabCBAcaDA");
    }

    #[test]
    fn chars_match_works_correctly() {
        assert!(!chars_match(None, 'a'));
        assert!(!chars_match(Some('b'), 'a'));
        assert!(!chars_match(Some('B'), 'a'));
        assert!(!chars_match(Some('b'), 'A'));
        assert!(!chars_match(Some('a'), 'a'));
        assert!(!chars_match(Some('A'), 'A'));
        assert!(chars_match(Some('A'), 'a'));
        assert!(chars_match(Some('a'), 'A'));
    }
}
