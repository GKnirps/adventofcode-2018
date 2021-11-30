use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let puzzle1_result = puzzle1(&content);
    println!("result: {}", puzzle1_result);

    let puzzle2_result = puzzle2(&content);
    println!("result: {}", puzzle2_result);

    Ok(())
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

fn puzzle1(input: &str) -> i64 {
    input
        .split('\n')
        .filter_map(|s| s.parse::<i64>().ok())
        .sum()
}

fn puzzle2(input: &str) -> i64 {
    let mut current: i64 = 0;
    let mut seen: HashSet<i64> = HashSet::with_capacity(100);
    seen.insert(current);
    for n in input
        .split('\n')
        .filter_map(|s| s.parse::<i64>().ok())
        .cycle()
    {
        current = current + n;
        if seen.contains(&current) {
            return current;
        }
        seen.insert(current);
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle1_correctly() {
        assert_eq!(puzzle1(""), 0);
        assert_eq!(puzzle1("+1\n+2\n+3"), 6);
        assert_eq!(puzzle1("+1\n+2\n-3"), 0);
        assert_eq!(puzzle1("+1\n+2\n-4\n"), -1);
    }

    #[test]
    fn puzzle2_correctly() {
        assert_eq!(puzzle2("+1\n-1"), 0);
        assert_eq!(puzzle2("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(puzzle2("-6\n+3\n+8\n+5\n-6\n"), 5);
        assert_eq!(puzzle2("+7\n+7\n-2\n-7\n-4\n"), 14);
    }
}
