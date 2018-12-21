use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Expression {
    Dir(Direction),
    Group(Vec<Expression>),
    Or(Box<Expression>, Box<Expression>),
}

fn doors_from_expression(re: &Expression) -> HashSet<(i32, i32, i32, i32)> {
    // visited rooms, shortest path length so far
    let mut doors: HashSet<(i32, i32, i32, i32)> = HashSet::with_capacity(1024);
    doors_sub(0, 0, re, &mut doors);
    return doors;
}

fn doors_sub(
    px: i32,
    py: i32,
    re: &Expression,
    doors: &mut HashSet<(i32, i32, i32, i32)>,
) -> HashSet<(i32, i32)> {
    return match re {
        Expression::Dir(direction) => {
            let (x_next, y_next) = match direction {
                Direction::North => (px, py - 1),
                Direction::East => (px + 1, py),
                Direction::South => (px, py + 1),
                Direction::West => (px - 1, py),
            };
            doors.insert((px, py, x_next, y_next));
            doors.insert((x_next, y_next, px, py));
            [(x_next, y_next)].into_iter().cloned().collect()
        }
        Expression::Or(re1, re2) => {
            let result1 = doors_sub(px, py, re1, doors);
            let result2 = doors_sub(px, py, re2, doors);
            result1.union(&result2).cloned().collect()
        }
        Expression::Group(expressions) => {
            let mut ends: HashSet<(i32, i32)> = [(px, py)].into_iter().cloned().collect();
            for expression in expressions {
                let mut new_ends: HashSet<(i32, i32)> = HashSet::with_capacity(256);
                for (x, y) in ends {
                    new_ends = new_ends
                        .union(&doors_sub(x, y, expression, doors))
                        .cloned()
                        .collect();
                }
                ends = new_ends;
            }
            ends
        }
    };
}

fn explore(
    doors: &HashSet<(i32, i32, i32, i32)>,
    start_x: i32,
    start_y: i32,
) -> HashMap<(i32, i32), u32> {
    let mut visited: HashMap<(i32, i32), u32> = HashMap::with_capacity(doors.len());
    let mut queue: VecDeque<(i32, i32, u32)> = VecDeque::with_capacity(doors.len());
    queue.push_back((start_x, start_y, 0));
    visited.insert((start_x, start_y), 0);
    while let Some((x, y, dist)) = queue.pop_front() {
        if doors.contains(&(x, y, x - 1, y)) && !visited.contains_key(&(x - 1, y)) {
            queue.push_back((x - 1, y, dist + 1));
            visited.insert((x - 1, y), dist + 1);
        }
        if doors.contains(&(x, y, x + 1, y)) && !visited.contains_key(&(x + 1, y)) {
            queue.push_back((x + 1, y, dist + 1));
            visited.insert((x + 1, y), dist + 1);
        }
        if doors.contains(&(x, y, x, y - 1)) && !visited.contains_key(&(x, y - 1)) {
            queue.push_back((x, y - 1, dist + 1));
            visited.insert((x, y - 1), dist + 1);
        }
        if doors.contains(&(x, y, x, y + 1)) && !visited.contains_key(&(x, y + 1)) {
            queue.push_back((x, y + 1, dist + 1));
            visited.insert((x, y + 1), dist + 1);
        }
    }
    return visited;
}

fn furthest_room(doors: &HashSet<(i32, i32, i32, i32)>, start_x: i32, start_y: i32) -> u32 {
    return explore(doors, start_x, start_y)
        .values()
        .max()
        .cloned()
        .unwrap_or(0);
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let expression = parse_input(&content)?;

    let doors = doors_from_expression(&expression);
    let furthest_dist = furthest_room(&doors, 0, 0);
    println!(
        "The shortest path to the furthest room has {} doors.",
        furthest_dist
    );

    Ok(())
}

fn parse_input(input: &str) -> Result<Expression, String> {
    let mut tokens = input.chars().filter(|c| !c.is_whitespace());
    let mut stack: Vec<char> = Vec::with_capacity(input.len());
    return parse_expression(&mut tokens, &mut stack);
}

fn parse_expression(
    input: &mut Iterator<Item = char>,
    stack: &mut Vec<char>,
) -> Result<Expression, String> {
    let mut group: Vec<Expression> = Vec::with_capacity(16);
    while let Some(token) = input.next() {
        match token {
            // just ignore those for now and assume the expression is valid
            '^' => (),
            '$' => (),
            'N' => group.push(Expression::Dir(Direction::North)),
            'E' => group.push(Expression::Dir(Direction::East)),
            'S' => group.push(Expression::Dir(Direction::South)),
            'W' => group.push(Expression::Dir(Direction::West)),
            '(' => {
                stack.push('(');
                group.push(parse_expression(input, stack)?);
            }
            ')' => {
                if stack.pop() != Some('(') {
                    return Err("Unexpected closing parenthesis".to_owned());
                }
                return Ok(Expression::Group(group));
            }
            '|' => {
                return Ok(Expression::Or(
                    Box::new(Expression::Group(group)),
                    Box::new(parse_expression(input, stack)?),
                ));
            }
            _ => {
                return Err("Unexpected token".to_owned());
            }
        }
    }
    return Ok(Expression::Group(group));
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn furthest_room_should_work_for_example() {
        // given
        let re_str = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let re = parse_input(re_str).unwrap();

        // when
        let doors = doors_from_expression(&re);
        let max_dist = furthest_room(&doors, 0, 0);

        // then
        assert_eq!(max_dist, 31);
    }
}
