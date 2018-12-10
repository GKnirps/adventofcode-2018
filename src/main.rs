#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();
    let points = parse_input(&lines);

    Ok(())
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

fn parse_input(lines: &[&str]) -> Vec<Point> {
    lines.iter().filter_map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Option<Point> {
    lazy_static! {
        static ref RE_POINT: Regex =
            Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
                .unwrap();
    }
    let capture = RE_POINT.captures(line)?;
    let pos_x: i32 = capture.get(1)?.as_str().parse().ok()?;
    let pos_y: i32 = capture.get(2)?.as_str().parse().ok()?;
    let vel_x: i32 = capture.get(3)?.as_str().parse().ok()?;
    let vel_y: i32 = capture.get(4)?.as_str().parse().ok()?;

    return Some(Point {
        position: (pos_x, pos_y),
        velocity: (vel_x, vel_y),
    });
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_line_works_correctly() {
        // given
        let input = "position=<-20620, -41485> velocity=< 2,  4>";

        // when
        let p = parse_line(input).expect("Expected a point");

        // then
        assert_eq!(p.position, (-20620, -41485));
        assert_eq!(p.velocity, (2, 4));
    }
}
