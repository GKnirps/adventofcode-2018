use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let mut points = parse_input(&content);
    if points.is_empty() {
        return Err("Expected points.".to_owned());
    }

    let mut timer = 0;
    while height(&points) > 20 {
        move_points(&mut points);
        timer += 1;
    }
    while height(&points) <= 20 {
        println!("-----------time: {}----------------", timer);
        print_points(&points);
        move_points(&mut points);
        timer += 1;
    }

    Ok(())
}

fn height(points: &[Point]) -> i32 {
    let (_, lower_y, _, upper_y) = get_bounds(points);
    upper_y - lower_y + 1
}

fn move_points(points: &mut [Point]) {
    for p in points {
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
        println!();
    }
}

fn get_bounds(points: &[Point]) -> (i32, i32, i32, i32) {
    let lower_x = points.iter().map(|p| p.position.0).min().unwrap();
    let lower_y = points.iter().map(|p| p.position.1).min().unwrap();
    let upper_x = points.iter().map(|p| p.position.0).max().unwrap();
    let upper_y = points.iter().map(|p| p.position.1).max().unwrap();

    (lower_x, lower_y, upper_x, upper_y)
}

fn parse_input(input: &str) -> Vec<Point> {
    input.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<Point> {
    let (pos, vel) = line.split_once("> velocity=<")?;

    let (pos_x, pos_y) = pos.strip_prefix("position=<")?.split_once(", ")?;
    let pos_x: i32 = pos_x.trim().parse().ok()?;
    let pos_y: i32 = pos_y.trim().parse().ok()?;

    let (vel_x, vel_y) = vel.strip_suffix('>')?.split_once(", ")?;
    let vel_x: i32 = vel_x.trim().parse().ok()?;
    let vel_y: i32 = vel_y.trim().parse().ok()?;

    Some(Point {
        position: (pos_x, pos_y),
        velocity: (vel_x, vel_y),
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
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
