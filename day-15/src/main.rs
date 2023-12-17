use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fmt;
use std::fs::read_to_string;
use std::path::Path;

static GOB_ATTACK_POWER: u32 = 3;

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
            self.tiles[x + y * self.xs] = tile;
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
    enemies
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.ys {
            for x in 0..self.xs {
                write!(f, "{}", self.tiles[x + y * self.xs])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// return the direction where to go for the next enemy
// if there are multiple enemies in the same range, take the one first in read order
// if an enemy is on an adjacent tile, return the direction to that tile
// if no path to any enemy is available, return None
// if no entity is at the given position, return None
fn shortest_path_to_enemy(px: usize, py: usize, cavern: &Cavern) -> Option<(usize, usize)> {
    let current_entity = cavern.get(px, py).and_then(|t| t.get_entity())?;
    // set of all visited positions with predecessors
    let mut visited: HashMap<(usize, usize), Option<(usize, usize)>> =
        HashMap::with_capacity(cavern.xs * cavern.ys);
    // queue of positions to check, with (px, py, distance to predecessor)
    let mut queue: VecDeque<(usize, usize, usize)> =
        VecDeque::with_capacity(2 * (cavern.xs + cavern.ys));
    // candidates for the closest enemy (px, py, id)
    let mut candidates: Vec<(usize, usize)> = Vec::with_capacity(10);
    // distance of the closest enemy
    let mut enemy_distance: Option<usize> = None;

    visited.insert((px, py), None);
    queue.push_back((px, py, 0));

    while let Some((x, y, distance)) = queue.pop_front() {
        // once we found an enemy, we only need to search all tiles that are no more distant than this enemy
        if enemy_distance.map(|d| d < distance).unwrap_or(false) {
            break;
        }

        for (ex, ey, _) in adjacent_enemies(cavern, current_entity.side, x, y) {
            if let std::collections::hash_map::Entry::Vacant(e) = visited.entry((ex, ey)) {
                candidates.push((ex, ey));
                e.insert(Some((x, y)));
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
    let (target_x, target_y) = get_best_candidate(&candidates)?;
    let mut current_pos = get_attack_side_for_candidate(target_x, target_y, &visited)?;
    while let Some(next_pos) = visited.get(&current_pos).and_then(|p| *p) {
        prev_pos = Some(current_pos);
        current_pos = next_pos;
    }
    prev_pos
}

fn get_attack_side_for_candidate(
    x: usize,
    y: usize,
    visited: &HashMap<(usize, usize), Option<(usize, usize)>>,
) -> Option<(usize, usize)> {
    if y > 0 && visited.contains_key(&(x, y - 1)) {
        return Some((x, y - 1));
    }
    if x > 0 && visited.contains_key(&(x - 1, y)) {
        return Some((x - 1, y));
    }
    if visited.contains_key(&(x + 1, y)) {
        return Some((x + 1, y));
    }
    if visited.contains_key(&(x, y + 1)) {
        return Some((x, y + 1));
    }
    None
}

fn get_best_candidate(candidates: &[(usize, usize)]) -> Option<(usize, usize)> {
    candidates
        .iter()
        .min_by(|(x1, y1), (x2, y2)| y1.cmp(y2).then(x1.cmp(x2)))
        .map(|(x, y)| (*x, *y))
}

fn best_target_in_range(cavern: &Cavern, x: usize, y: usize) -> Option<(usize, usize)> {
    let current_entity = cavern.get_entity(x, y)?;
    let adj_enemies = adjacent_enemies(cavern, current_entity.side, x, y);
    let lowest_health = adj_enemies.iter().map(|(_, _, e)| e.health).min()?;
    let lowest_health_enemies: Vec<(usize, usize)> = adj_enemies
        .iter()
        .filter(|(_, _, e)| e.health == lowest_health)
        .map(|(x, y, _)| (*x, *y))
        .collect();
    get_best_candidate(&lowest_health_enemies)
}

fn next_round(mut cavern: Cavern, elf_attack_power: u32) -> (Cavern, bool) {
    let entities = cavern.entities_in_reading_order();

    for (x, y, id) in &entities {
        let attack_power = if let Some(entity) = cavern.get_entity(*x, *y) {
            if entity.id != *id {
                // entity died and a different entity is here now. This entity already had its
                // turn, ignore it
                continue;
            }
            // only end the round prematurely if the unit looking for an enemy is still alive
            if cavern.n_goblins == 0 || cavern.n_elves == 0 {
                return (cavern, false);
            }
            match entity.side {
                Side::Elf => elf_attack_power,
                Side::Goblin => GOB_ATTACK_POWER,
            }
        } else {
            // unit is dead
            continue;
        };
        if let Some((target_x, target_y)) = best_target_in_range(&cavern, *x, *y) {
            attack_tile(&mut cavern, target_x, target_y, attack_power);
        } else if let Some((next_x, next_y)) = shortest_path_to_enemy(*x, *y, &cavern) {
            cavern.move_entity(*x, *y, next_x, next_y);
            if let Some((target_x, target_y)) = best_target_in_range(&cavern, next_x, next_y) {
                attack_tile(&mut cavern, target_x, target_y, attack_power);
            }
        }
    }

    (cavern, true)
}

fn attack_tile(cavern: &mut Cavern, target_x: usize, target_y: usize, attack_power: u32) {
    if let Some(enemy) = cavern.get_entity(target_x, target_y) {
        if enemy.health <= attack_power {
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
                    health: enemy.health - attack_power,
                })),
            );
        }
    }
}

fn fight(mut cavern: Cavern, elf_attack_power: u32) -> (Cavern, u32) {
    let mut rounds: u32 = 0;
    let mut round_finished = true;
    while round_finished {
        let result = next_round(cavern, elf_attack_power);
        cavern = result.0;
        round_finished = result.1;
        if round_finished {
            rounds += 1;
        }
    }
    (cavern, rounds)
}

fn cheat_until_elves_win(cavern: Cavern) -> (Cavern, u32, u32) {
    let mut elf_attack_power: u32 = 4;
    loop {
        let (final_cavern, rounds) = fight(cavern.clone(), elf_attack_power);
        if final_cavern.n_elves == cavern.n_elves {
            return (final_cavern, rounds, elf_attack_power);
        }
        elf_attack_power += 1;
    }
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
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();
    let initial_cavern = parse_cavern(&lines)?;

    let (final_cavern, rounds) = fight(initial_cavern.clone(), 3);
    let health_sum = sum_health(&final_cavern);
    println!("Outcome of the battle: rounds: {}, remaining health: {}, surviving goblins: {}, surviving elves: {}, outcome value: {}", rounds, health_sum, final_cavern.n_goblins, final_cavern.n_elves, rounds * health_sum);

    let (final_cheat_cavern, cheat_rounds, required_elf_attack_power) =
        cheat_until_elves_win(initial_cavern.clone());
    let cheated_health_sum = sum_health(&final_cheat_cavern);
    println!("Elves win after {} rounds without losses, remaining health: {}. Required attack power: {}. Outcome value: {}", cheat_rounds, cheated_health_sum, required_elf_attack_power, cheat_rounds * cheated_health_sum);

    Ok(())
}

fn parse_cavern(lines: &[&str]) -> Result<Cavern, String> {
    let ys = lines.iter().filter(|l| !l.is_empty()).count();
    let xs = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let tiles: Vec<Tile> = lines
        .iter()
        .filter(|l| !l.is_empty())
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
    Ok(Cavern {
        tiles,
        xs,
        ys,
        n_elves,
        n_goblins,
    })
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
        cavern = next_round(cavern, 3).0;
        println!("After 1 round:\n{}", cavern);
        cavern = next_round(cavern, 3).0;
        println!("After 2 rounds:\n{}", cavern);
        cavern = next_round(cavern, 3).0;
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

    #[test]
    fn combat_works_for_example_3() {
        let lines = &[
            "#######", "#E..EG#", "#.#G.E#", "#E.##E#", "#G..#.#", "#..E#.#", "#######",
        ];
        run_combat_test(lines, 46, 859);
    }

    #[test]
    fn combat_works_for_example_4() {
        let lines = &[
            "#######", "#E.G#.#", "#.#G..#", "#G.#.G#", "#G..#.#", "#...E.#", "#######",
        ];
        run_combat_test(lines, 35, 793);
    }
    #[test]
    fn combat_works_for_example_5() {
        let lines = &[
            "#######", "#.E...#", "#.#..G#", "#.###.#", "#E#G#G#", "#...#G#", "#######",
        ];
        run_combat_test(lines, 54, 536);
    }
    #[test]
    fn combat_works_for_example_6() {
        let lines = &[
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
        ];
        run_combat_test(lines, 20, 937);
    }

    fn run_combat_test(raw_cavern: &[&str], expected_rounds: u32, expected_health: u32) {
        // given
        let initial_cavern = parse_cavern(raw_cavern).expect("Expected valid cavern");

        // when
        println!("initial cavern:\n{}", initial_cavern);
        let (final_cavern, rounds) = fight(initial_cavern, 3);
        let health_sum = sum_health(&final_cavern);
        println!("final cavern:\n{}", final_cavern);

        // then
        assert_eq!(rounds, expected_rounds);
        assert_eq!(health_sum, expected_health);
    }

    #[test]
    fn cheating_works_for_example_1() {
        let lines = &[
            "#######", "#.G...#", "#...EG#", "#.#.#G#", "#..G#E#", "#.....#", "#######",
        ];
        let cavern = run_cheating_test(lines, 29, 172, 15);

        let elf1 = cavern.get_entity(3, 1).expect("Expected first elf");
        assert_eq!(elf1.health, 158);

        let elf2 = cavern.get_entity(4, 2).expect("Expected second elf");
        assert_eq!(elf2.health, 14);
    }

    #[test]
    fn cheating_works_for_example_2() {
        let lines = &[
            "#######", "#E..EG#", "#.#G.E#", "#E.##E#", "#G..#.#", "#..E#.#", "#######",
        ];
        run_cheating_test(lines, 33, 948, 4);
    }

    #[test]
    fn cheating_works_for_example_3() {
        let lines = &[
            "#######", "#E.G#.#", "#.#G..#", "#G.#.G#", "#G..#.#", "#...E.#", "#######",
        ];
        run_cheating_test(lines, 37, 94, 15);
    }

    #[test]
    fn cheating_works_for_example_4() {
        let lines = &[
            "#######", "#.E...#", "#.#..G#", "#.###.#", "#E#G#G#", "#...#G#", "#######",
        ];
        run_cheating_test(lines, 39, 166, 12);
    }

    #[test]
    fn cheating_works_for_example_5() {
        run_cheating_test(
            &[
                "#########",
                "#G......#",
                "#.E.#...#",
                "#..##..G#",
                "#...##..#",
                "#...#...#",
                "#.G...G.#",
                "#.....G.#",
                "#########",
            ],
            30,
            38,
            34,
        );
    }

    fn run_cheating_test(
        raw_cavern: &[&str],
        expected_rounds: u32,
        expected_health: u32,
        expected_power: u32,
    ) -> Cavern {
        let initial_cavern = parse_cavern(raw_cavern).expect("Expected valid cavern");
        let initial_elves = initial_cavern.n_elves;

        // when
        println!("initial cavern:\n{}", initial_cavern);
        let (final_cavern, rounds, power) = cheat_until_elves_win(initial_cavern);
        let health_sum = sum_health(&final_cavern);
        println!("final cavern:\n{}", final_cavern);

        // then
        assert_eq!(final_cavern.n_goblins, 0);
        assert_eq!(final_cavern.n_elves, initial_elves);
        assert_eq!(rounds, expected_rounds);
        assert_eq!(health_sum, expected_health);
        assert_eq!(power, expected_power);

        final_cavern
    }

    #[test]
    fn test_attack_preferences_if_health_is_equal() {
        // given
        let initial_cavern = parse_cavern(&[".G.", "GEG", ".G."]).expect("Expected valid cavern");

        // when
        // 200 attack power, so each hit is a kill for elves
        let (cavern1, _) = next_round(initial_cavern, 200);
        println!("after round 1:\n{}", cavern1);
        let (cavern2, _) = next_round(cavern1.clone(), 200);
        println!("after round 2:\n{}", cavern2);
        let (cavern3, _) = next_round(cavern2.clone(), 200);
        println!("after round 3:\n{}", cavern3);
        let (cavern4, _) = next_round(cavern3.clone(), 200);
        println!("after round 4:\n{}", cavern4);

        // then
        assert_eq!(cavern1.get_entity(1, 0), None);
        assert!(cavern1.get_entity(0, 1).is_some());
        assert!(cavern1.get_entity(2, 1).is_some());
        assert!(cavern1.get_entity(1, 2).is_some());

        assert_eq!(cavern2.get_entity(1, 0), None);
        assert_eq!(cavern2.get_entity(0, 1), None);
        assert!(cavern2.get_entity(2, 1).is_some());
        assert!(cavern2.get_entity(1, 2).is_some());

        assert_eq!(cavern3.get_entity(1, 0), None);
        assert_eq!(cavern3.get_entity(0, 1), None);
        assert_eq!(cavern3.get_entity(2, 1), None);
        assert!(cavern3.get_entity(1, 2).is_some());

        assert_eq!(cavern4.get_entity(1, 0), None);
        assert_eq!(cavern4.get_entity(0, 1), None);
        assert_eq!(cavern4.get_entity(2, 1), None);
        assert_eq!(cavern4.get_entity(1, 2), None);
    }

    #[test]
    fn test_attack_preferences_for_lower_health() {
        // given
        let mut initial_cavern = parse_cavern(&["GEG"]).expect("Expected valid cavern");
        initial_cavern.set(
            2,
            0,
            Tile::Open(Some(Entity {
                id: 2,
                side: Side::Goblin,
                health: 199,
            })),
        );

        // when
        // 200 attack power, so each hit is a kill for elves
        let (cavern, _) = next_round(initial_cavern, 200);
        println!("after round 1:\n{}", cavern);

        // then
        assert!(cavern.get_entity(0, 0).is_some());
        assert_eq!(cavern.get_entity(0, 2), None);
    }

    #[test]
    fn round_ends_prematurely_if_no_enemies_are_left() {
        // given
        let initial_cavern = parse_cavern(&["GEE"]).expect("Expected valid cavern");

        // when
        // 200 attack power, so each hit is a kill for elves
        let (cavern_1, finished_round_1) = next_round(initial_cavern, 200);
        let (cavern_2, finished_round_2) = next_round(cavern_1.clone(), 200);

        // then
        assert!(!finished_round_1);
        assert!(!finished_round_2);
        assert_eq!(cavern_1, cavern_2);
    }

    #[test]
    fn round_does_not_end_prematurely_if_last_entities_turn_kills_last_enemy() {
        // given
        let initial_cavern = parse_cavern(&["EEG"]).expect("Expected valid cavern");

        // when
        // 200 attack power, so each hit is a kill for elves
        let (cavern_1, finished_round_1) = next_round(initial_cavern, 200);
        let (cavern_2, finished_round_2) = next_round(cavern_1.clone(), 200);

        // then
        assert!(finished_round_1);
        assert!(!finished_round_2);
        assert_eq!(cavern_1, cavern_2);
    }

    #[test]
    fn movement_selection_with_tricky_obstacles_works_correctly() {
        // given
        // tricky, because the elf should select the tile left to the goblin
        // as tile to attack from (first in reading order), but from the elf's position, up
        // is the first tile in reading order
        let lines = &[
            "......#", ".####.#", "E####.#", ".##.G.#", ".##.###", ".##.###", ".......",
        ];
        let initial_cavern = parse_cavern(lines).expect("Expected valid cavern");

        // when
        let (cavern, _) = next_round(initial_cavern, 3);

        // then
        assert!(cavern.get_entity(0, 1).is_none());
        assert!(cavern.get_entity(0, 3).is_some());
    }
}
