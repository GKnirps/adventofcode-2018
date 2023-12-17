use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;

    let points = parse(&content)?;

    let n = num_constellations(&points);
    println!("{n} constellations are formed by the fixed points in spacetime");

    Ok(())
}

fn num_constellations(points: &[Point]) -> usize {
    // let's just brute force it
    let mut constellations: Vec<HashSet<Point>> = points
        .iter()
        .map(|point| {
            let mut constellation: HashSet<Point> = HashSet::with_capacity(points.len());
            constellation.insert(*point);
            constellation
        })
        .collect();

    let mut n_constellations = constellations.len() + 1;

    while n_constellations != constellations.len() {
        n_constellations = constellations.len();
        for i in 0..constellations.len() {
            for j in (i + 1)..constellations.len() {
                if constellations_connect(&constellations[i], &constellations[j]) {
                    // a trick to get two mutable borrows to different parts of the array
                    let (part1, part2) = constellations.split_at_mut(j);
                    part1[i].extend(part2[0].drain());
                }
            }
        }
        constellations.retain(|c| !c.is_empty());
    }

    constellations.len()
}

fn constellations_connect(const1: &HashSet<Point>, const2: &HashSet<Point>) -> bool {
    const1
        .iter()
        .any(|p1| const2.iter().any(|p2| distance(*p1, *p2) <= 3))
}

fn distance(p1: Point, p2: Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs() + (p1.3 - p2.3).abs()
}

type Point = (i32, i32, i32, i32);

fn parse(input: &str) -> Result<Vec<Point>, String> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Point, String> {
    let mut numbers = line.split(',').map(|num| {
        num.parse::<i32>()
            .map_err(|e| format!("unable to parse number '{num}':{e}"))
    });
    let point = (
        numbers
            .next()
            .ok_or_else(|| format!("not enough numbers in line '{line}'"))??,
        numbers
            .next()
            .ok_or_else(|| format!("not enough numbers in line '{line}'"))??,
        numbers
            .next()
            .ok_or_else(|| format!("not enough numbers in line '{line}'"))??,
        numbers
            .next()
            .ok_or_else(|| format!("not enough numbers in line '{line}'"))??,
    );
    if numbers.next().is_some() {
        Err(format!("too many numbersin line '{line}'"))
    } else {
        Ok(point)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
"#;

    #[test]
    fn num_constellations_works_for_example() {
        // given
        let points = parse(EXAMPLE).expect("expected successful parsing");

        // when
        let n = num_constellations(&points);

        // then
        assert_eq!(n, 4);
    }
}
