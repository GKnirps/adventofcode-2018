#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

type Position = (i64, i64, i64);

fn dist(p1: &Position, p2: &Position) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bot {
    pos: Position,
    radius: i64,
}

fn bots_in_range_of_strongest_bot(bots: &[Bot]) -> usize {
    if let Some(strongest_bot) = bots.iter().max_by_key(|b| b.radius) {
        return bots
            .iter()
            .map(|bot| dist(&strongest_bot.pos, &bot.pos))
            .filter(|d| *d <= strongest_bot.radius)
            .count();
    }
    return 0;
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();

    let bots = parse_bots(&lines)?;

    let n_in_range = bots_in_range_of_strongest_bot(&bots);
    println!(
        "There are {} bots in range of the bots with the strongest signal.",
        n_in_range
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

    #[test]
    fn bots_in_range_of_strongest_bot_works_for_example() {
        // given
        let lines = &[
            "pos=<0,0,0>, r=4",
            "pos=<1,0,0>, r=1",
            "pos=<4,0,0>, r=3",
            "pos=<0,2,0>, r=1",
            "pos=<0,5,0>, r=3",
            "pos=<0,0,3>, r=1",
            "pos=<1,1,1>, r=1",
            "pos=<1,1,2>, r=1",
            "pos=<1,3,1>, r=1",
        ];
        let bots = parse_bots(lines).expect("Expected valid bots");

        // when
        let n = bots_in_range_of_strongest_bot(&bots);

        // then
        assert_eq!(n, 7);
    }
}
