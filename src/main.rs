use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let reacted = react_polymer(content.trim());

    println!(
        "After reacting, the polymer has a length of {} units",
        reacted.chars().count()
    );

    Ok(())
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

fn react_polymer(pol: &str) -> String {
    let mut stack: Vec<char> = Vec::with_capacity(pol.len());
    for c in pol.chars() {
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
        let result = react_polymer(input);

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
