use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use std::env;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    
    let puzzle1_result = puzzle1(&content);
    println!("result: {}", puzzle1_result);

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
    input.split('\n').filter_map(|s| s.parse::<i64>().ok()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle1_correctly() {
        assert_eq!(calc(""), 0);
        assert_eq!(calc("+1\n+2\n+3"), 6);
        assert_eq!(calc("+1\n+2\n-3"), 0);
        assert_eq!(calc("+1\n+2\n-4\n"), -1);
    }
}
