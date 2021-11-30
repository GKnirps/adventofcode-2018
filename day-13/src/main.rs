use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TrackPiece {
    Horizontal,
    Vertical,
    TurnSlash,
    TurnBSlash,
    Intersection,
}

impl TrackPiece {
    fn from_char(c: char) -> Option<TrackPiece> {
        match c {
            '-' => Some(TrackPiece::Horizontal),
            '<' => Some(TrackPiece::Horizontal),
            '>' => Some(TrackPiece::Horizontal),
            '^' => Some(TrackPiece::Vertical),
            'v' => Some(TrackPiece::Vertical),
            '|' => Some(TrackPiece::Vertical),
            '/' => Some(TrackPiece::TurnSlash),
            '\\' => Some(TrackPiece::TurnBSlash),
            '+' => Some(TrackPiece::Intersection),
            _ => None,
        }
    }
}

type Tracks = HashMap<(usize, usize), TrackPiece>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cart {
    px: usize,
    py: usize,
    dir: Direction,
    turn: u8,
}

fn cmp_cart_pos(left: &Cart, right: &Cart) -> Ordering {
    if left.py < right.py {
        Ordering::Less
    } else if left.py > right.py {
        Ordering::Greater
    } else {
        left.px.cmp(&right.px)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CartsTick {
    Success(Vec<Cart>),
    Crashed(usize, usize),
}

fn next_tick(mut carts: Vec<Cart>, tracks: &Tracks) -> Result<CartsTick, String> {
    carts.sort_by(cmp_cart_pos);
    for i in 0..carts.len() {
        carts[i] = move_cart(&carts[i], tracks)?;
        for j in 0..i {
            if carts[j].px == carts[i].px && carts[j].py == carts[i].py {
                return Ok(CartsTick::Crashed(carts[i].px, carts[i].py));
            }
        }
        for j in i + 1..carts.len() {
            if carts[j].px == carts[i].px && carts[j].py == carts[i].py {
                return Ok(CartsTick::Crashed(carts[i].px, carts[i].py));
            }
        }
    }
    return Ok(CartsTick::Success(carts));
}

fn next_tick_remove_crashed(mut carts: Vec<Cart>, tracks: &Tracks) -> Result<Vec<Cart>, String> {
    // That is some very nice mutable data structure you have there. Would be a shame if anything happened to it…
    carts.sort_by(cmp_cart_pos);
    let mut next_carts: Vec<Cart> = Vec::with_capacity(carts.len());
    let mut i: usize = 0;
    while i < carts.len() {
        let cart = &carts[i];
        let next_cart = move_cart(cart, tracks)?;
        let dup_index = carts[i + 1..]
            .iter()
            .enumerate()
            .filter(|(_, c)| c.px == next_cart.px && c.py == next_cart.py)
            .map(|(i, _)| i)
            .next();
        if let Some(i) = dup_index {
            carts.remove(i);
        } else if next_carts
            .iter()
            .any(|c| c.px == next_cart.px && c.py == next_cart.py)
        {
            next_carts.retain(|c| c.px != next_cart.px || c.py != next_cart.py);
        } else {
            next_carts.push(next_cart);
        }
        i += 1;
    }
    return Ok(next_carts);
}

fn move_cart(cart: &Cart, tracks: &Tracks) -> Result<Cart, String> {
    if cart.dir == Direction::Left && cart.px == 0 || cart.dir == Direction::Up && cart.py == 0 {
        return Err(format!(
            "A cart was leaving the area in direction {:?} at position {}×{}.",
            cart.dir, cart.px, cart.py
        ));
    }
    let (next_x, next_y) = match cart.dir {
        Direction::Left => (cart.px - 1, cart.py),
        Direction::Right => (cart.px + 1, cart.py),
        Direction::Up => (cart.px, cart.py - 1),
        Direction::Down => (cart.px, cart.py + 1),
    };

    let next_track = tracks.get(&(next_x, next_y)).ok_or_else(|| {
        format!(
            "A cart left the rails from {}×{} to {}×{}",
            cart.px, cart.py, next_x, next_y
        )
    })?;

    let (dir, turn) = next_dir_for_cart(cart, &next_track);
    return Ok(Cart {
        px: next_x,
        py: next_y,
        dir,
        turn,
    });
}

fn next_dir_for_cart(cart: &Cart, track: &TrackPiece) -> (Direction, u8) {
    match track {
        TrackPiece::Horizontal => (cart.dir, cart.turn),
        TrackPiece::Vertical => (cart.dir, cart.turn),
        TrackPiece::TurnSlash => (
            match cart.dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            cart.turn,
        ),
        TrackPiece::TurnBSlash => (
            match cart.dir {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            cart.turn,
        ),
        TrackPiece::Intersection => {
            (match cart.turn % 3 {
                0 => (turn_left(cart.dir), 1),
                1 => (cart.dir, 2),
                _ => (turn_right(cart.dir), 0),
            })
        }
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
    }
}
fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn run_until_crash(tracks: &Tracks, mut carts: Vec<Cart>) -> Result<(usize, usize), String> {
    loop {
        carts = match next_tick(carts, tracks)? {
            CartsTick::Crashed(px, py) => {
                return Ok((px, py));
            }
            CartsTick::Success(cart) => cart,
        }
    }
}

fn there_can_be_only_one(tracks: &Tracks, mut carts: Vec<Cart>) -> Result<(usize, usize), String> {
    while carts.len() > 1 {
        carts = next_tick_remove_crashed(carts, tracks)?;
    }
    return carts
        .get(0)
        .map(|cart| (cart.px, cart.py))
        .ok_or_else(|| "All carts crashed!".to_owned());
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();
    let (tracks, carts) = parse_map(&lines);

    let (crash_x, crash_y) = run_until_crash(&tracks, carts.clone())?;
    println!(
        "The first crash occurs at position {}×{}",
        crash_x, crash_y
    );

    let (last_x, last_y) = there_can_be_only_one(&tracks, carts)?;
    println!(
        "The last surviving cart is at position {}×{}",
        last_x, last_y
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

fn parse_map(lines: &[&str]) -> (Tracks, Vec<Cart>) {
    let mut tracks: Tracks =
        Tracks::with_capacity(lines.len() * lines.get(0).map(|l| l.len()).unwrap_or(0));
    let mut carts: Vec<Cart> = Vec::with_capacity(128);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(track_piece) = TrackPiece::from_char(c) {
                tracks.insert((x, y), track_piece);
            }
            if let Some(dir) = Direction::from_char(c) {
                carts.push(Cart {
                    dir,
                    px: x,
                    py: y,
                    turn: 0,
                });
            }
        }
    }
    return (tracks, carts);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn crashtest_works_for_example() {
        // given
        let lines = &[
            r"/->-\        ",
            r"|   |  /----\",
            r"| /-+--+-\  |",
            r"| | |  | v  |",
            r"\-+-/  \-+--/",
            r"  \------/   ",
        ];
        let (tracks, carts) = parse_map(lines);

        // when
        let (crash_x, crash_y) = run_until_crash(&tracks, carts).unwrap();

        // then
        assert_eq!(crash_x, 7);
        assert_eq!(crash_y, 3);
    }

    #[test]
    fn last_survivor_works_for_example() {
        // given
        let lines = &[
            r"/>-<\  ", r"|   |  ", r"| /<+-\", r"| | | v", r"\>+</ |", r"  |   ^", r"  \<->/",
        ];
        let (tracks, carts) = parse_map(lines);

        // when
        let (last_x, last_y) = there_can_be_only_one(&tracks, carts).unwrap();

        // then
        assert_eq!(last_x, 6);
        assert_eq!(last_y, 4);
    }
}
