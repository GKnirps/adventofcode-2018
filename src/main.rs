use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fmt;
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Open(Option<Entity>),
}

impl Tile {
    fn get_entity(&self) -> Option<&Entity> {
        match self {
            Tile::Open(Some(ent)) => Some(ent),
            _ => None,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Open(None) => write!(f, "."),
            Tile::Open(Some(Entity {
                id: _,
                health: _,
                side: Side::Elf,
            })) => write!(f, "E"),
            Tile::Open(Some(Entity {
                id: _,
                health: _,
                side: Side::Goblin,
            })) => write!(f, "G"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cavern {
    xs: usize,
    ys: usize,
    tiles: Vec<Tile>,
    n_goblins: usize,
    n_elves: usize,
}

impl Cavern {
    // Find all entities and return them in reading order as (px, py, id)
    fn entities_in_reading_order(&self) -> Vec<(usize, usize, usize)> {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(i, t)| t.get_entity().map(|e| (i % self.xs, i / self.xs, e.id)))
            .collect()
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(x + y * self.xs)
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.xs && y < self.ys {
            self.tiles[x + y * self.ys] = tile;
        }
    }

    fn move_entity(&mut self, x: usize, y: usize, next_x: usize, next_y: usize) {
        if self.get(next_x, next_y) == Some(&Tile::Open(None))
            && self
                .get(x, y)
                .map(|t| t.get_entity().is_some())
                .unwrap_or(false)
        {
            self.tiles.swap(x + self.xs * y, next_x + self.xs * next_y);
        }
    }

    fn get_entity(&self, x: usize, y: usize) -> Option<&Entity> {
        self.tiles.get(x + y * self.xs).and_then(|p| p.get_entity())
    }

    // return true iff a tile is valid, open space and not occupied
    fn tile_free(&self, x: usize, y: usize) -> bool {
        self.get(x, y)
            .map(|t| t == &Tile::Open(None))
            .unwrap_or(false)
    }
}

fn adjacent_enemies(
    cavern: &Cavern,
    side: Side,
    x: usize,
    y: usize,
) -> Vec<(usize, usize, &Entity)> {
    let mut enemies: Vec<(usize, usize, &Entity)> = Vec::new();
    if y > 0 {
        if let Some(enemy) = cavern.get_entity(x, y - 1).filter(|e| e.side != side) {
            enemies.push((x, y - 1, enemy));
        }
    }
    if x > 0 {
        if let Some(enemy) = cavern.get_entity(x - 1, y).filter(|e| e.side != side) {
            enemies.push((x - 1, y, enemy));
        }
    }
    if let Some(enemy) = cavern.get_entity(x + 1, y).filter(|e| e.side != side) {
        enemies.push((x + 1, y, enemy));
    }
    if let Some(enemy) = cavern.get_entity(x, y + 1).filter(|e| e.side != side) {
        enemies.push((x, y + 1, enemy));
    }
    return enemies;
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.ys {
            for x in 0..self.xs {
                write!(f, "{}", self.tiles[x + y * self.xs])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// return the direction where to go for the next enemy
// if there are multiple enemies in the same range, take the one first in read order
// if an enemy is on an adjacent tile, return the direction to that tile
// if no path to any enemy is available, return None
// if no entity is at the given position, return None
fn shortest_path_to_enemy(
    px: usize,
    py: usize,
    cavern: &Cavern,
    read_order: &[(usize, usize, usize)],
) -> Option<(usize, usize)> {
    let current_entity = cavern.get(px, py).and_then(|t| t.get_entity())?;
    // set of all visited positions with predecessors
    let mut visited: HashMap<(usize, usize), Option<(usize, usize)>> =
        HashMap::with_capacity(cavern.xs * cavern.ys);
    // queue of positions to check, with (px, py, distance to predecessor)
    let mut queue: VecDeque<(usize, usize, usize)> =
        VecDeque::with_capacity(2 * (cavern.xs + cavern.ys));
    // candidates for the closest enemy (px, py, id)
    let mut candidates: Vec<(usize, usize, usize)> = Vec::with_capacity(10);
    // distance of the closest enemy
    let mut enemy_distance: Option<usize> = None;

    visited.insert((px, py), None);
    queue.push_back((px, py, 0));

    while let Some((x, y, distance)) = queue.pop_front() {
        // once we found an enemy, we only need to search all tiles that are no more distant than this enemy
        if enemy_distance.map(|d| d < distance).unwrap_or(false) {
            break;
        }

        for (ex, ey, entity) in adjacent_enemies(&cavern, current_entity.side, x, y) {
            if !visited.contains_key(&(ex, ey)) {
                candidates.push((ex, ey, entity.id));
                visited.insert((ex, ey), Some((x, y)));
                enemy_distance = Some(distance);
            }
        }
        // push all adjacent, unvisited, open and empty tiles to the queue and mark them as visited
        if y > 0 && cavern.tile_free(x, y - 1) && !visited.contains_key(&(x, y - 1)) {
            visited.insert((x, y - 1), Some((x, y)));
            queue.push_back((x, y - 1, distance + 1));
        }
        if x > 0 && cavern.tile_free(x - 1, y) && !visited.contains_key(&(x - 1, y)) {
            visited.insert((x - 1, y), Some((x, y)));
            queue.push_back((x - 1, y, distance + 1));
        }
        if cavern.tile_free(x + 1, y) && !visited.contains_key(&(x + 1, y)) {
            visited.insert((x + 1, y), Some((x, y)));
            queue.push_back((x + 1, y, distance + 1));
        }
        if cavern.tile_free(x, y + 1) && !visited.contains_key(&(x, y + 1)) {
            visited.insert((x, y + 1), Some((x, y)));
            queue.push_back((x, y + 1, distance + 1));
        }
    }
    // backtrack the path
    let mut prev_pos: Option<(usize, usize)> = None;
    let mut current_pos = get_best_candidate(&candidates, read_order)?;
    while let Some(next_pos) = visited.get(&current_pos).and_then(|p| *p) {
        prev_pos = Some(current_pos);
        current_pos = next_pos;
    }
    return prev_pos;
}

fn get_best_candidate(
    candidates: &[(usize, usize, usize)],
    read_order: &[(usize, usize, usize)],
) -> Option<(usize, usize)> {
    candidates
        .iter()
        .filter_map(|(cx, cy, id)| {
            let order_index = read_order
                .iter()
                .enumerate()
                .filter(|(_, (_, _, o_id))| id == o_id)
                .map(|(index, _)| index)
                .next()?;
            return Some((cx, cy, order_index));
        })
        .min_by_key(|(_, _, index)| *index)
        .map(|(x, y, _)| (*x, *y))
}

fn best_target_in_range(
    cavern: &Cavern,
    entities: &[(usize, usize, usize)],
    x: usize,
    y: usize,
) -> Option<(usize, usize)> {
    let current_entity = cavern.get_entity(x, y)?;
    let adj_enemies = adjacent_enemies(cavern, current_entity.side, x, y);
    let lowest_health = adj_enemies.iter().map(|(_, _, e)| e.health).min()?;
    let lowest_health_enemies: Vec<(usize, usize, usize)> = adj_enemies
        .iter()
        .filter(|(_, _, e)| e.health == lowest_health)
        .map(|(x, y, e)| (*x, *y, e.id))
        .collect();
    return get_best_candidate(&lowest_health_enemies, entities);
}

fn next_round(mut cavern: Cavern) -> (Cavern, bool) {
    let entities = cavern.entities_in_reading_order();

    for (x, y, _) in &entities {
        if cavern.n_goblins == 0 || cavern.n_elves == 0 {
            return (cavern, false);
        }
        if let Some((target_x, target_y)) = best_target_in_range(&cavern, &entities, *x, *y) {
            attack_tile(&mut cavern, target_x, target_y);
        } else if let Some((next_x, next_y)) = shortest_path_to_enemy(*x, *y, &cavern, &entities) {
            cavern.move_entity(*x, *y, next_x, next_y);
            if let Some((target_x, target_y)) =
                best_target_in_range(&cavern, &entities, next_x, next_y)
            {
                attack_tile(&mut cavern, target_x, target_y);
            }
        }
    }

    return (cavern, true);
}

fn attack_tile(cavern: &mut Cavern, target_x: usize, target_y: usize) {
    if let Some(enemy) = cavern.get_entity(target_x, target_y) {
        if enemy.health <= ATTACK_POWER {
            match enemy.side {
                Side::Elf => cavern.n_elves -= 1,
                Side::Goblin => cavern.n_goblins -= 1,
            }
            cavern.set(target_x, target_y, Tile::Open(None));
        } else {
            cavern.set(
                target_x,
                target_y,
                Tile::Open(Some(Entity {
                    id: enemy.id,
                    side: enemy.side,
                    health: enemy.health - ATTACK_POWER,
                })),
            );
        }
    }
}

fn fight(mut cavern: Cavern) -> (Cavern, u32) {
    let mut rounds: u32 = 0;
    let mut round_finished = true;
    while round_finished {
        let result = next_round(cavern);
        cavern = result.0;
        round_finished = result.1;
        if round_finished {
            rounds += 1;
        }
    }
    return (cavern, rounds);
}

fn sum_health(cavern: &Cavern) -> u32 {
    cavern
        .tiles
        .iter()
        .filter_map(|t| t.get_entity())
        .map(|e| e.health)
        .sum()
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();
    let initial_cavern = parse_cavern(&lines)?;

    let (final_cavern, rounds) = fight(initial_cavern);
    let health_sum = sum_health(&final_cavern);
    println!("Outcome of the battle: rounds: {}, remaining health: {}, surviving goblins: {}, surviving elves: {}, outcome value: {}", rounds, health_sum, final_cavern.n_goblins, final_cavern.n_elves, rounds * health_sum);

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
    let n_elves = tiles
        .iter()
        .filter_map(|t| t.get_entity())
        .filter(|e| e.side == Side::Elf)
        .count();
    let n_goblins = tiles
        .iter()
        .filter_map(|t| t.get_entity())
        .filter(|e| e.side == Side::Goblin)
        .count();
    return Ok(Cavern {
        tiles,
        xs,
        ys,
        n_elves,
        n_goblins,
    });
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

    #[test]
    fn reading_order_works_for_example() {
        // given
        let lines = &["#######", "#.G.E.#", "#E.G.E#", "#.G.E.#", "#######"];
        let cavern = parse_cavern(lines).expect("Expected valid cavern");

        // when
        let order = cavern.entities_in_reading_order();

        // then
        assert_eq!(cavern.n_goblins, 3);
        assert_eq!(cavern.n_elves, 4);
        assert_eq!(order.len(), 7);
        assert_eq!(order[0], (2, 1, 9));
        assert_eq!(order[1], (4, 1, 11));
        assert_eq!(order[2], (1, 2, 15));
        assert_eq!(order[3], (3, 2, 17));
        assert_eq!(order[4], (5, 2, 19));
        assert_eq!(order[5], (2, 3, 23));
        assert_eq!(order[6], (4, 3, 25));
    }

    #[test]
    fn movement_works_for_example() {
        // given
        let lines = &[
            "#########",
            "#G..G..G#",
            "#.......#",
            "#.......#",
            "#G..E..G#",
            "#.......#",
            "#.......#",
            "#G..G..G#",
            "#########",
        ];
        let mut cavern = parse_cavern(lines).expect("Expected valid cavern");

        // when
        cavern = next_round(cavern).0;
        println!("After 1 round:\n{}", cavern);
        cavern = next_round(cavern).0;
        println!("After 2 rounds:\n{}", cavern);
        cavern = next_round(cavern).0;
        println!("After 3 rounds:\n{}", cavern);

        // then
        let goblin1 = cavern.get_entity(3, 2).expect("Expected first goblin");
        assert_eq!(goblin1.side, Side::Goblin);
        assert_eq!(goblin1.id, 10);

        let goblin2 = cavern.get_entity(4, 2).expect("Expected second goblin");
        assert_eq!(goblin2.side, Side::Goblin);
        assert_eq!(goblin2.id, 13);

        let goblin3 = cavern.get_entity(5, 2).expect("Expected third goblin");
        assert_eq!(goblin3.side, Side::Goblin);
        assert_eq!(goblin3.id, 16);

        let goblin4 = cavern.get_entity(3, 3).expect("Expected fourth goblin");
        assert_eq!(goblin4.side, Side::Goblin);
        assert_eq!(goblin4.id, 37);

        let elf = cavern.get_entity(4, 3).expect("Expected elf");
        assert_eq!(elf.side, Side::Elf);
        assert_eq!(elf.id, 40);

        let goblin5 = cavern.get_entity(5, 3).expect("Expected fifth goblin");
        assert_eq!(goblin5.side, Side::Goblin);
        assert_eq!(goblin5.id, 43);

        let goblin6 = cavern.get_entity(1, 4).expect("Expected sixth goblin");
        assert_eq!(goblin6.side, Side::Goblin);
        assert_eq!(goblin6.id, 64);

        let goblin7 = cavern.get_entity(4, 4).expect("Expected seventh goblin");
        assert_eq!(goblin7.side, Side::Goblin);
        assert_eq!(goblin7.id, 67);

        let goblin8 = cavern.get_entity(7, 5).expect("Expected eighth goblin");
        assert_eq!(goblin8.side, Side::Goblin);
        assert_eq!(goblin8.id, 70);
    }

    #[test]
    fn combat_works_for_example_1() {
        run_combat_test(
            &[
                "#######", "#.G...#", "#...EG#", "#.#.#G#", "#..G#E#", "#.....#", "#######",
            ],
            47,
            590,
        );
    }

    #[test]
    fn combat_works_for_example_2() {
        run_combat_test(
            &[
                "#######", "#G..#E#", "#E#E.E#", "#G.##.#", "#...#E#", "#...E.#", "#######",
            ],
            37,
            982,
        );
    }

    fn run_combat_test(raw_cavern: &[&str], expected_rounds: u32, expected_health: u32) {
        // given
        let initial_cavern = parse_cavern(raw_cavern).expect("Expected valid cavern");

        // when
        println!("initial cavern:\n{}", initial_cavern);
        let (final_cavern, rounds) = fight(initial_cavern);
        let health_sum = sum_health(&final_cavern);
        println!("final cavern:\n{}", final_cavern);

        // then
        assert_eq!(rounds, expected_rounds);
        assert_eq!(health_sum, expected_health);
    }
}
