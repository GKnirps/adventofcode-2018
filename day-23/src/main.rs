use std::env;
use std::fs::read_to_string;
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
    0
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cube {
    pos: Position,
    side_length: i64,
}

// return a fitting cube that contains the positions of all bots
// or None if there are no bots
fn fitting_cube(bots: &[Bot]) -> Option<Cube> {
    let x_min: i64 = bots.iter().map(|b| b.pos.0 - b.radius).min()?;
    let y_min: i64 = bots.iter().map(|b| b.pos.1 - b.radius).min()?;
    let z_min: i64 = bots.iter().map(|b| b.pos.2 - b.radius).min()?;

    let x_len: i64 = bots.iter().map(|b| b.pos.0 + b.radius).max()? - x_min;
    let y_len: i64 = bots.iter().map(|b| b.pos.1 + b.radius).max()? - y_min;
    let z_len: i64 = bots.iter().map(|b| b.pos.2 + b.radius).max()? - z_min;

    let side_length: i64 = x_len.max(y_len).max(z_len);

    Some(Cube {
        pos: (x_min, y_min, z_min),
        side_length,
    })
}

fn point_in_cube(cube: &Cube, point: &Position) -> bool {
    cube.pos.0 <= point.0
        && cube.pos.1 <= point.1
        && cube.pos.2 <= point.2
        && cube.pos.0 + cube.side_length > point.0
        && cube.pos.1 + cube.side_length > point.1
        && cube.pos.2 + cube.side_length > point.2
}

fn bot_in_range(cube: &Cube, bot: &Bot) -> bool {
    let cube_range = cube.side_length - 1;
    point_in_cube(cube, &(bot.pos.0 + bot.radius, bot.pos.1, bot.pos.2))
        || point_in_cube(cube, &(bot.pos.0 - bot.radius, bot.pos.1, bot.pos.2))
        || point_in_cube(cube, &(bot.pos.0, bot.pos.1 + bot.radius, bot.pos.2))
        || point_in_cube(cube, &(bot.pos.0, bot.pos.1 - bot.radius, bot.pos.2))
        || point_in_cube(cube, &(bot.pos.0, bot.pos.1, bot.pos.2 + bot.radius))
        || point_in_cube(cube, &(bot.pos.0, bot.pos.1, bot.pos.2 - bot.radius))
        || point_in_cube(cube, &bot.pos)
        || dist(&cube.pos, &bot.pos) <= bot.radius
        || dist(&(cube.pos.0 + cube_range, cube.pos.1, cube.pos.2), &bot.pos) <= bot.radius
        || dist(&(cube.pos.0, cube.pos.1 + cube_range, cube.pos.2), &bot.pos) <= bot.radius
        || dist(&(cube.pos.0, cube.pos.1, cube.pos.2 + cube_range), &bot.pos) <= bot.radius
        || dist(
            &(cube.pos.0 + cube_range, cube.pos.1 + cube_range, cube.pos.2),
            &bot.pos,
        ) <= bot.radius
        || dist(
            &(cube.pos.0 + cube_range, cube.pos.1, cube.pos.2 + cube_range),
            &bot.pos,
        ) <= bot.radius
        || dist(
            &(cube.pos.0, cube.pos.1 + cube_range, cube.pos.2 + cube_range),
            &bot.pos,
        ) <= bot.radius
        || dist(
            &(
                cube.pos.0 + cube_range,
                cube.pos.1 + cube_range,
                cube.pos.2 + cube_range,
            ),
            &bot.pos,
        ) <= bot.radius
}

fn bots_in_range(cube: &Cube, bots: &[Bot]) -> u64 {
    bots.iter().filter(|bot| bot_in_range(cube, bot)).count() as u64
}

fn split_cube(cube: &Cube) -> [Cube; 8] {
    let new_side = cube.side_length / 2 + cube.side_length % 2;
    let (x, y, z) = cube.pos;
    [
        Cube {
            pos: cube.pos,
            side_length: new_side,
        },
        Cube {
            pos: (x + new_side, y, z),
            side_length: new_side,
        },
        Cube {
            pos: (x, y + new_side, z),
            side_length: new_side,
        },
        Cube {
            pos: (x, y, z + new_side),
            side_length: new_side,
        },
        Cube {
            pos: (x + new_side, y + new_side, z),
            side_length: new_side,
        },
        Cube {
            pos: (x + new_side, y, z + new_side),
            side_length: new_side,
        },
        Cube {
            pos: (x, y + new_side, z + new_side),
            side_length: new_side,
        },
        Cube {
            pos: (x + new_side, y + new_side, z + new_side),
            side_length: new_side,
        },
    ]
}

fn find_best_positions(bots: &[Bot]) -> Vec<Position> {
    // idea: first, create a cube that contains all bots
    // then, partition that cube into 8 smaller cubes and only continue
    // with the cube(s) that are in reach of the most bots
    // continue until the cubes have a side length of 1

    let mut current_cubes: Vec<Cube> = fitting_cube(bots).into_iter().collect();
    // FIXME
    println!(
        "Starting with cube {:?}, with bots in range: {}",
        current_cubes,
        bots_in_range(&current_cubes[0], bots)
    );
    while current_cubes.iter().map(|c| c.side_length).any(|s| s > 1) {
        // split up cubes with the number of bots that can reach them
        let mut next_cubes: Vec<(Cube, u64)> = Vec::with_capacity(current_cubes.len() * 8);
        for cube in current_cubes {
            // ignore cubes with a side length of 1 for now (they cannot be split up anymore)
            if cube.side_length == 1 {
                next_cubes.push((cube.clone(), bots_in_range(&cube, bots)));
            } else {
                for smaller in split_cube(&cube) {
                    let in_range = bots_in_range(&smaller, bots);
                    next_cubes.push((smaller, in_range));
                }
            }
        }

        let max_bots = next_cubes.iter().map(|(_, n)| *n).max();
        // only the cubes with the maximum number of bot that reach them continue
        current_cubes = next_cubes
            .into_iter()
            .filter(|(_, n)| Some(*n) == max_bots)
            .map(|(c, _)| c)
            .collect();
        // FIXME
        println!(
            "Next iteration: {} cubes, maximal number of reachable bots: {:?}, side length: {}",
            current_cubes.len(),
            max_bots,
            current_cubes[0].side_length
        );
    }
    current_cubes.into_iter().map(|c| c.pos).collect()
}

fn closest_to_origin(positions: &[Position]) -> Option<Position> {
    positions
        .iter()
        .max_by_key(|p| dist(p, &(0, 0, 0)))
        .cloned()
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.lines().collect();

    let bots = parse_bots(&lines)?;

    let n_in_range = bots_in_range_of_strongest_bot(&bots);
    println!(
        "There are {} bots in range of the bots with the strongest signal.",
        n_in_range
    );

    let best_positions = find_best_positions(&bots);
    println!("There are {} optimal positions.", bots.len());
    let closest_best = closest_to_origin(&best_positions).ok_or("No best positions!".to_owned())?;
    println!(
        "Closest optimal position is {}×{}×{} with a manhattan distance of {}",
        closest_best.0,
        closest_best.1,
        closest_best.2,
        dist(&closest_best, &(0, 0, 0))
    );

    Ok(())
}

fn parse_bots(lines: &[&str]) -> Result<Vec<Bot>, String> {
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| parse_bot(l).ok_or_else(|| format!("Unable to parse line as nanobot: '{}'", l)))
        .collect()
}

fn parse_bot(line: &str) -> Option<Bot> {
    let (pos, radius) = line.split_once(">, r=")?;
    let mut pos = pos.strip_prefix("pos=<")?.splitn(3, ',');

    let px: i64 = pos.next()?.parse().ok()?;
    let py: i64 = pos.next()?.parse().ok()?;
    let pz: i64 = pos.next()?.parse().ok()?;
    let radius: i64 = radius.parse().ok()?;

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

    #[test]
    fn find_best_position_works_for_example() {
        // given
        let lines = &[
            "pos=<10,12,12>, r=2",
            "pos=<12,14,12>, r=2",
            "pos=<16,12,12>, r=4",
            "pos=<14,14,14>, r=6",
            "pos=<50,50,50>, r=200",
            "pos=<10,10,10>, r=5",
        ];
        let bots = parse_bots(lines).expect("Expected valid bots");

        // when
        let best_positions = find_best_positions(&bots);
        let closest_best =
            closest_to_origin(&best_positions).expect("Expected at least one optimal position");

        // then
        assert_eq!(closest_best, (12, 12, 12));
    }
}
