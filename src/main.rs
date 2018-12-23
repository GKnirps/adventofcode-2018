#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

type Position = (i64, i64, i64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bot {
    pos: Position,
    radius: i64,
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();

    let bots = parse_bots(&lines)?;

    Ok(())
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

fn parse_bots(lines: &[&str]) -> Result<Vec<Bot>, String> {
    lines
        .iter()
        .filter(|l| l.len() > 0)
        .map(|l| parse_bot(l).ok_or_else(|| format!("Unable to parse line as nanobot: '{}'", l)))
        .collect()
}

fn parse_bot(line: &str) -> Option<Bot> {
    lazy_static! {
        static ref RE_BOT: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)")
            .expect("Expected valid bot regex");
    }
    let capture = RE_BOT.captures(line)?;
    let px: i64 = capture.get(1)?.as_str().parse().ok()?;
    let py: i64 = capture.get(2)?.as_str().parse().ok()?;
    let pz: i64 = capture.get(3)?.as_str().parse().ok()?;
    let radius: i64 = capture.get(4)?.as_str().parse().ok()?;

    Some(Bot {
        pos: (px, py, pz),
        radius,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_bot_works_for_actual_line() {
        // given
        let line = "pos=<-25859315,11930330,30505051>, r=55054958";

        // when
        let bot = parse_bot(line).expect("Expected valid bot");

        // then
        assert_eq!(bot.pos, (-25859315, 11930330, 30505051));
        assert_eq!(bot.radius, 55054958);
    }
}
