use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let lines = split_lines(&content);

    let (twos, threes) = count_multiples(&lines);
    println!(
        "twos: {}, threes: {}, checksum: {}",
        twos,
        threes,
        twos * threes
    );

    if let Some(id) = matching_ids(&lines) {
        println!("ID {} is the match", &id);
    } else {
        println!("Apparently, no IDs match.");
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

fn split_lines<'a>(input: &'a str) -> Vec<&'a str> {
    input.split('\n').collect()
}

fn count_multiples(ids: &[&str]) -> (u64, u64) {
    ids.iter()
        .map(|s| count_letters(s))
        .map(|m| has_multiples(&m))
        .fold((0, 0), |(twos, threes), (two, three)| {
            (
                twos + if two { 1 } else { 0 },
                threes + if three { 1 } else { 0 },
            )
        })
}

fn has_multiples(counter: &HashMap<char, u64>) -> (bool, bool) {
    return (
        counter.values().any(|count| count == &2),
        counter.values().any(|count| count == &3),
    );
}

fn count_letters(id: &str) -> HashMap<char, u64> {
    let mut result = HashMap::with_capacity(id.len());
    for c in id.chars() {
        if let Some(count) = result.get(&c) {
            result.insert(c, count + 1);
        } else {
            result.insert(c, 1);
        }
    }
    return result;
}

fn strings_differ_by_one(a: &str, b: &str) -> Option<String> {
    if a.len() != b.len() {
        return None;
    }
    let diff = a
        .chars()
        .zip(b.chars())
        .filter(|(left, right)| left != right)
        .count();
    return if diff != 1 {
        None
    } else {
        Some(
            a.chars()
                .zip(b.chars())
                .filter(|(left, right)| left == right)
                .map(|(l, _)| l)
                .collect(),
        )
    };
}

fn matching_ids(ids: &[&str]) -> Option<String> {
    for i in 0..(ids.len() - 1) {
        for j in (i + 1)..ids.len() {
            if let Some(id) = strings_differ_by_one(ids[i], ids[j]) {
                return Some(id);
            }
        }
    }
    return None;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_letters_counts_letters() {
        // given
        let input = "ababc";

        // when
        let result = count_letters(input);

        // then
        assert_eq!(result.len(), 3);
        assert_eq!(result.get(&'a'), Some(&2));
        assert_eq!(result.get(&'b'), Some(&2));
        assert_eq!(result.get(&'c'), Some(&1));
    }

    #[test]
    fn has_multiples_returns_correct_values() {
        assert_eq!(has_multiples(&count_letters("")), (false, false));
        assert_eq!(has_multiples(&count_letters("ac")), (false, false));
        assert_eq!(has_multiples(&count_letters("abac")), (true, false));
        assert_eq!(has_multiples(&count_letters("abbbc")), (false, true));
        assert_eq!(has_multiples(&count_letters("abbbac")), (true, true));
    }

    #[test]
    fn count_multiples_works_correctly() {
        // given
        let input = [
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];

        // when
        let (twos, threes) = count_multiples(&input);

        // then
        assert_eq!(twos, 4);
        assert_eq!(threes, 3);
    }

    #[test]
    fn strings_differ_by_one_works_correctly() {
        assert_eq!(
            strings_differ_by_one("foobar", "foocar"),
            Some("fooar".to_owned())
        );
        assert_eq!(strings_differ_by_one("abcde", "axcye"), None);
        assert_eq!(strings_differ_by_one("abcde", "abcdef"), None);
    }

    #[test]
    fn matching_ids_finds_matching_ids() {
        // given
        let ids = [
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];

        // when
        let result = matching_ids(&ids).unwrap();

        // then
        assert_eq!(&result, "fgij");
    }

    #[test]
    fn matching_ids_returns_none_for_no_matching_ids() {
        // given
        let ids = ["abcde", "fghij", "klmno", "pqrst", "axcye", "wvxyz"];

        // when
        let result = matching_ids(&ids);

        // then
        assert!(result.is_none());
    }
}
