use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

static ATTACK_POWER: u32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    Elf,
    Goblin,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Entity {
    id: usize,
    side: Side,
    health: u32,
}

impl Entity {
    fn new_elf(id: usize) -> Entity {
        Entity {
            id,
            side: Side::Elf,
            health: 200,
        }
    }
    fn new_gob(id: usize) -> Entity {
        Entity {
            id,
            side: Side::Goblin,
            health: 200,
        }
    }
    fn is_enemy(&self, other: &Entity) -> bool {
        self.side != other.side
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Open(Option<Entity>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cavern {
    xs: usize,
    ys: usize,
    tiles: Vec<Tile>,
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();
    let initial_cavern = parse_cavern(&lines);

    Ok(())
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

fn parse_cavern(lines: &[&str]) -> Result<Cavern, String> {
    let ys = lines.iter().filter(|l| l.len() > 0).count();
    let xs = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let tiles: Vec<Tile> = lines
        .iter()
        .filter(|l| l.len() > 0)
        .flat_map(|line| line.chars())
        .enumerate()
        .map(|(i, c)| parse_tile(c, i))
        .collect();
    if tiles.len() != xs * ys {
        return Err(format!(
            "Bad Map: Expected {}×{} = {} tiles, but found {}.",
            xs,
            ys,
            xs * ys,
            tiles.len()
        ));
    }
    return Ok(Cavern { tiles, xs, ys });
}

fn parse_tile(tile: char, id: usize) -> Tile {
    match tile {
        '.' => Tile::Open(None),
        'E' => Tile::Open(Some(Entity::new_elf(id))),
        'G' => Tile::Open(Some(Entity::new_gob(id))),
        // actually, walls are just '#', but for simplicity, we interprete unknown chars as wall.
        _ => Tile::Wall,
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
