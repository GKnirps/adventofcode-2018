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
    let mut points = parse_input(&lines);
    if points.len() == 0 {
        return Err("Expected points.".to_owned());
    }

    while height(&points) > 20 {
        move_points(&mut points);
    }
    while height(&points) <= 20 {
        println!("---------------------------");
        print_points(&points);
        move_points(&mut points);
    }

    Ok(())
}

fn height(points: &[Point]) -> i32 {
    let (_, lower_y, _, upper_y) = get_bounds(points);
    return upper_y - lower_y + 1;
}

fn move_points(points: &mut [Point]) {
    for mut p in points {
        p.position.0 += p.velocity.0;
        p.position.1 += p.velocity.1;
    }
}

fn print_points(points: &[Point]) {
    let (lower_x, lower_y, upper_x, upper_y) = get_bounds(points);
    for y in lower_y..(upper_y + 1) {
        for x in lower_x..(upper_x + 1) {
            if points
                .iter()
                .any(|p| p.position.0 == x && p.position.1 == y)
            {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn get_bounds(points: &[Point]) -> (i32, i32, i32, i32) {
    let lower_x = points.iter().map(|p| p.position.0).min().unwrap();
    let lower_y = points.iter().map(|p| p.position.1).min().unwrap();
    let upper_x = points.iter().map(|p| p.position.0).max().unwrap();
    let upper_y = points.iter().map(|p| p.position.1).max().unwrap();;

    (lower_x, lower_y, upper_x, upper_y)
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
