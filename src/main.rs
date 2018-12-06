use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();
    let coords = parse_coords(&lines);

    let world = calc_areas(&coords);
    let infinite_areas = world.get_areas_at_border();
    let area_sizes = calc_area_sizes(&world, coords.len());

    let max_finite_area_size = area_sizes
        .iter()
        .enumerate()
        .filter(|(i, _)| !infinite_areas.contains(&i))
        .max_by_key(|(_, size)| size.clone());

    if let Some((index, size)) = max_finite_area_size {
        println!(
            "Area {} is the largest finite area with a size of {}",
            index, size
        );
    } else {
        println!("There are no finite areas");
    }

    let puzzle2_area = area_with_max_distance_sum(&coords, 10000);
    println!("Solution for puzzle 2: {}", puzzle2_area);

    Ok(())
}

// this is rather inefficient, as I have to cover a region with about 4 billion cells
// I probably should have started from the inside and gone outside until I reached a limit
// maybe some other day...
fn area_with_max_distance_sum(coords_param: &[Coord], max_distance: usize) -> usize {
    // Great: I have usize everywhere, but we may need negative numbers here
    // because I will never need this code again, I can just shift all the coordinates instead...
    let coords: Vec<Coord> = coords_param
        .iter()
        .map(|coord| Coord {
            x: coord.x + max_distance,
            y: coord.y + max_distance,
        })
        .collect();
    if coords.len() == 0 {
        return 0;
    }
    // I know this unwrap is dirty, but I'm tired and I will never use this code again
    let upper_x = coords.iter().min_by_key(|c| c.x.clone()).unwrap().x + max_distance;
    let upper_y = coords.iter().min_by_key(|c| c.y.clone()).unwrap().y + max_distance;
    let lower_x = coords.iter().max_by_key(|c| c.x.clone()).unwrap().x - max_distance;
    let lower_y = coords.iter().max_by_key(|c| c.y.clone()).unwrap().y - max_distance;

    println!(
        "Lower bounds: {}×{}, upper bounds: {}×{}",
        lower_x, lower_y, upper_x, upper_y
    );
    if lower_x >= upper_x || lower_y >= upper_y {
        return 0;
    }
    let mut count = 0;
    for y in lower_y..upper_y {
        for x in lower_x..upper_x {
            let location = Coord { x, y };
            let total_distance: usize = coords
                .iter()
                .map(|c| manhattan_distance(c, &location))
                .sum();
            if total_distance < max_distance {
                count += 1;
            }
        }
    }
    return count;
}

fn manhattan_distance(p1: &Coord, p2: &Coord) -> usize {
    let dist_x = if p1.x > p2.x {
        p1.x - p2.x
    } else {
        p2.x - p1.x
    };
    let dist_y = if p1.y > p2.y {
        p1.y - p2.y
    } else {
        p2.y - p1.y
    };
    return dist_x + dist_y;
}

fn calc_area_sizes(world: &World, n_areas: usize) -> Vec<u32> {
    let mut result: Vec<u32> = (0..n_areas).map(|_| 0).collect();

    for cell in &world.cells {
        if let Some(index) = cell {
            result[*index] += 1;
        }
    }

    return result;
}

fn calc_areas(coords: &[Coord]) -> World {
    let mut world = World::from_coordinates(coords);
    let mut changed = true;
    while changed {
        let (w, c) = world.grow();
        world = w;
        changed = c;
    }
    return world;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct World {
    cells: Vec<Option<usize>>,
    xs: usize,
    ys: usize,
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.ys {
            for x in 0..self.xs {
                if let Some(i) = self.get(x, y) {
                    write!(f, "{}", i)?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl World {
    fn new(xs: usize, ys: usize) -> World {
        World {
            xs,
            ys,
            cells: (0..(xs * ys)).map(|_| None).collect(),
        }
    }

    fn from_coordinates(coords: &[Coord]) -> World {
        // size is determined by the highest value in each direction
        // I know I could also ignore everything smaller than the lowest values,
        // but I don't want to deal with coordinate shift, and the values are reasonably
        // small, so it's not a performance issue
        let xs = coords.iter().map(|c| c.x).max().unwrap_or(0) + 1;
        let ys = coords.iter().map(|c| c.y).max().unwrap_or(0) + 1;
        let mut world = World::new(xs, ys);

        for (i, Coord { x, y }) in coords.iter().enumerate() {
            world.set(*x, *y, i);
        }

        return world;
    }

    // grow all areas according to the rules
    // return the new world state and a boolean that is true iff the world changed
    fn grow(&self) -> (World, bool) {
        let mut result = self.clone();
        let mut changed = false;
        for y in 0..self.ys {
            for x in 0..self.xs {
                if self.get(x, y).is_none() {
                    if let Some(value) = self.sole_neighbour(x, y) {
                        changed = true;
                        result.set(x, y, value);
                    }
                }
            }
        }
        return (result, changed);
    }

    fn get(&self, x: usize, y: usize) -> Option<usize> {
        return self.cells.get(x + y * self.xs).and_then(|v| v.clone());
    }

    fn sole_neighbour(&self, x: usize, y: usize) -> Option<usize> {
        let neighbours = self.get_neighbours(x, y);
        let (_, sole) = neighbours.iter().filter_map(|n| n.clone()).fold(
            (true, None),
            |(free, last_value), value| {
                if free && (last_value.is_none() || last_value == Some(value)) {
                    return (true, Some(value));
                }
                return (false, None);
            },
        );
        return sole;
    }

    fn get_neighbours(&self, x: usize, y: usize) -> [Option<usize>; 4] {
        let left = if x == 0 { None } else { self.get(x - 1, y) };
        let up = if y == 0 { None } else { self.get(x, y - 1) };
        return [left, self.get(x + 1, y), up, self.get(x, y + 1)];
    }

    fn set(&mut self, x: usize, y: usize, value: usize) {
        if x < self.xs && y < self.ys {
            self.cells[x + y * self.xs] = Some(value);
        }
    }

    fn get_areas_at_border(&self) -> HashSet<usize> {
        let mut result = HashSet::with_capacity(self.xs * 2 + self.ys * 2 - 4);
        for x in 0..self.xs {
            if let Some(value) = self.get(x, 0) {
                result.insert(value);
            }
            // yes I know this will crash for 0-sized worlds, but I don't care
            if let Some(value) = self.get(x, self.ys - 1) {
                result.insert(value);
            }
        }
        for y in 0..self.ys {
            if let Some(value) = self.get(0, y) {
                result.insert(value);
            }
            if let Some(value) = self.get(self.xs - 1, y) {
                result.insert(value);
            }
        }
        return result;
    }
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

fn parse_coords(lines: &[&str]) -> Vec<Coord> {
    lines.iter().filter_map(|line| parse_coord(line)).collect()
}

fn parse_coord(line: &str) -> Option<Coord> {
    let mut splitted = line.split(", ");
    let x: usize = splitted.next()?.parse().ok()?;
    let y: usize = splitted.next()?.parse().ok()?;
    return Some(Coord { x, y });
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn manhattan_distance_works_correctly() {
        assert_eq!(
            manhattan_distance(&Coord { x: 0, y: 1 }, &Coord { x: 0, y: 1 }),
            0
        );
        assert_eq!(
            manhattan_distance(&Coord { x: 10, y: 100 }, &Coord { x: 20, y: 200 }),
            110
        );
        assert_eq!(
            manhattan_distance(&Coord { x: 20, y: 200 }, &Coord { x: 10, y: 100 }),
            110
        );
    }

    #[test]
    fn check_example_puzzle_2() {
        // given
        let coords = [
            Coord { x: 1, y: 1 },
            Coord { x: 1, y: 6 },
            Coord { x: 8, y: 3 },
            Coord { x: 3, y: 4 },
            Coord { x: 5, y: 5 },
            Coord { x: 8, y: 9 },
        ];
        let max_distance: usize = 32;

        // when
        let result = area_with_max_distance_sum(&coords, max_distance);

        // then
        assert_eq!(result, 16);
    }

    #[test]
    fn check_example_puzzle_1() {
        // given
        let coords = [
            Coord { x: 1, y: 1 },
            Coord { x: 1, y: 6 },
            Coord { x: 8, y: 3 },
            Coord { x: 3, y: 4 },
            Coord { x: 5, y: 5 },
            Coord { x: 8, y: 9 },
        ];

        // when
        let world = calc_areas(&coords);
        let infinite_areas = world.get_areas_at_border();
        let area_sizes = calc_area_sizes(&world, coords.len());

        // then
        assert_eq!(infinite_areas.len(), 4);
        assert!(infinite_areas.contains(&0));
        assert!(infinite_areas.contains(&1));
        assert!(infinite_areas.contains(&2));
        assert!(infinite_areas.contains(&5));
        assert_eq!(area_sizes[3], 9);
        assert_eq!(area_sizes[4], 17);
    }

    #[test]
    fn calc_area_sizes_returns_correct_area_sizes() {
        // given
        let xs = 3;
        let ys = 2;
        let n_areas = 2;
        let cells = vec![None, Some(0), Some(0), Some(0), Some(1), None];
        let world = World { xs, ys, cells };

        // when
        let sizes = calc_area_sizes(&world, n_areas);

        // then
        assert_eq!(sizes, vec![3, 1]);
    }

    #[test]
    fn world_get_areas_at_border_returns_areas_at_border() {
        // given
        let xs = 3;
        let ys = 3;
        //.0.
        //123
        //.4.
        let cells = vec![
            None,
            Some(0),
            None,
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(4),
            None,
        ];
        let world = World { xs, ys, cells };

        // when
        let areas = world.get_areas_at_border();

        // then
        assert_eq!(areas.len(), 4);
        assert!(areas.contains(&0));
        assert!(areas.contains(&1));
        assert!(areas.contains(&3));
        assert!(areas.contains(&4));
    }

    #[test]
    fn world_grow_area_does_not_rival_itself() {
        // given
        let xs = 3;
        let ys = 1;
        //0.0
        let cells = vec![Some(0), None, Some(0)];
        let world = World { xs, ys, cells };

        // when
        let (result, grew) = world.grow();

        // then
        assert!(grew);
        //000
        let expected_cells = vec![Some(0), Some(0), Some(0)];
        assert_eq!(result.cells, expected_cells);
    }

    #[test]
    fn world_grow_grows_correctly_with_rivals() {
        // given
        let xs = 3;
        let ys = 1;
        //0.1
        let cells = vec![Some(0), None, Some(1)];
        let world = World { xs, ys, cells };

        // when
        let (result, grew) = world.grow();

        // then
        assert!(!grew);
        assert_eq!(result.cells, world.cells);
    }

    #[test]
    fn world_grow_grows_correctly_at_border() {
        // given
        let xs = 3;
        let ys = 2;
        //.0.
        //...
        let cells = vec![None, Some(0), None, None, None, None];
        let world = World { xs, ys, cells };

        // when
        let (result, grew) = world.grow();

        // then
        assert!(grew);
        //000
        //.0.
        let expected_cells = vec![Some(0), Some(0), Some(0), None, Some(0), None];
        assert_eq!(result.cells, expected_cells);
    }

    #[test]
    fn world_from_coordinates_works_correctly() {
        // given
        let coords = [
            Coord { x: 1, y: 1 },
            Coord { x: 2, y: 4 },
            Coord { x: 3, y: 2 },
        ];

        // when
        let world = World::from_coordinates(&coords);

        // then
        assert_eq!(world.xs, 4);
        assert_eq!(world.ys, 5);
        assert_eq!(world.cells.len(), 20);
        assert_eq!(world.cells[1 + 4 * 1], Some(0));
        assert_eq!(world.cells[2 + 4 * 4], Some(1));
        assert_eq!(world.cells[3 + 4 * 2], Some(2));
        assert_eq!(world.cells.iter().filter(|c| c.is_none()).count(), 17);
    }

    #[test]
    fn parse_coords_parses_all_coordinates() {
        // given
        let input = ["1, 2", "30, 40", "500, 600"];

        // when
        let coords = parse_coords(&input);

        // then
        assert_eq!(coords.len(), 3);
        assert_eq!(coords.get(1), Some(&Coord { x: 30, y: 40 }));
    }

    #[test]
    fn parse_coord_parses_coordinates() {
        assert_eq!(parse_coord(""), None);
        assert_eq!(parse_coord("3, "), None);
        assert_eq!(parse_coord("foo, bar"), None);

        assert_eq!(parse_coord("42, 9001"), Some(Coord { x: 42, y: 9001 }));
    }
}
