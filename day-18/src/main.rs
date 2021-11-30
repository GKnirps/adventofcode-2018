use std::collections::{hash_map::Entry, HashMap};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Open,
    Trees,
    Lumberyard,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cell = match self {
            Cell::Open => '.',
            Cell::Trees => '|',
            Cell::Lumberyard => '#',
        };
        write!(f, "{}", cell)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Area {
    xs: usize,
    ys: usize,
    cells: Vec<Cell>,
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.ys {
            for x in 0..self.xs {
                write!(f, "{}", self.cells[x + y * self.xs])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Area {
    fn next_gen(&self) -> Area {
        let mut next_cells: Vec<Cell> = Vec::with_capacity(self.cells.len());
        for y in 0..self.ys {
            for x in 0..self.xs {
                next_cells.push(self.next_gen_cell(x, y));
            }
        }
        return Area {
            xs: self.xs,
            ys: self.ys,
            cells: next_cells,
        };
    }

    fn next_gen_cell(&self, x: usize, y: usize) -> Cell {
        let (_, n_trees, n_lumber) = self.neighbour_count(x, y);
        match self.cell(x as isize, y as isize) {
            Some(Cell::Open) => {
                if n_trees >= 3 {
                    Cell::Trees
                } else {
                    Cell::Open
                }
            }
            Some(Cell::Trees) => {
                if n_lumber >= 3 {
                    Cell::Lumberyard
                } else {
                    Cell::Trees
                }
            }
            Some(Cell::Lumberyard) => {
                if n_lumber > 0 && n_trees > 0 {
                    Cell::Lumberyard
                } else {
                    Cell::Open
                }
            }
            None => Cell::Open,
        }
    }

    fn neighbour_count(&self, x: usize, y: usize) -> (u8, u8, u8) {
        static POS: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let mut n_open = 0;
        let mut n_trees = 0;
        let mut n_lumber = 0;

        for (dx, dy) in &POS {
            let cell = self.cell(x as isize + dx, y as isize + dy);
            match cell {
                Some(Cell::Open) => {
                    n_open += 1;
                }
                Some(Cell::Trees) => {
                    n_trees += 1;
                }
                Some(Cell::Lumberyard) => n_lumber += 1,
                _ => (),
            };
        }
        return (n_open, n_trees, n_lumber);
    }

    fn cell(&self, x: isize, y: isize) -> Option<Cell> {
        if x < 0 || y < 0 || x >= self.xs as isize || y >= self.ys as isize {
            return None;
        }
        return self
            .cells
            .get(x as usize + y as usize * self.xs)
            .map(|c| *c);
    }

    fn count_cells(&self, cell_type: Cell) -> usize {
        self.cells.iter().filter(|c| **c == cell_type).count()
    }
}

fn after_generations(area: &Area, n_generations: usize) -> Rc<Area> {
    // a map of patterns to the index the pattern occured first
    let mut previous_patterns_index: HashMap<Rc<Area>, usize> = HashMap::with_capacity(1000);
    // a list of all patterns that have been observed so far, in order
    let mut previous_patterns: Vec<Rc<Area>> = Vec::with_capacity(1000);

    let mut current_area = Rc::new(area.clone());
    previous_patterns.push(Rc::clone(&current_area));
    previous_patterns_index.insert(Rc::clone(&current_area), 0);

    for n in 0..n_generations {
        let next_area = Rc::new(current_area.next_gen());
        previous_patterns.push(Rc::clone(&next_area));

        let entry = previous_patterns_index.entry(Rc::clone(&next_area));
        match entry {
            Entry::Vacant(v) => {
                v.insert(n + 1);
            }
            Entry::Occupied(p) => {
                let repetition_offset = *p.get();
                let cycle_length = n + 1 - repetition_offset;
                println!(
                    "Pattern #{} occured again after {} generations.",
                    repetition_offset, cycle_length
                );
                let end_index: usize =
                    repetition_offset + (n_generations - repetition_offset) % cycle_length;
                return Rc::clone(&previous_patterns[end_index]);
            }
        };
        current_area = next_area;
    }
    return current_area;
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let area = parse_area(&content)?;

    let after_10_minutes = after_generations(&area, 10);
    let tree_count = after_10_minutes.count_cells(Cell::Trees);
    let lumber_count = after_10_minutes.count_cells(Cell::Lumberyard);
    println!(
        "After 10 minutes: Trees: {}, Lumberyards: {}, resource value: {}",
        tree_count,
        lumber_count,
        tree_count * lumber_count
    );

    let after_billion_minutes = after_generations(&area, 1000000000);
    let tree_count = after_billion_minutes.count_cells(Cell::Trees);
    let lumber_count = after_billion_minutes.count_cells(Cell::Lumberyard);
    println!(
        "After 1000000000 minutes: Trees: {}, Lumberyards: {}, resource value: {}",
        tree_count,
        lumber_count,
        tree_count * lumber_count
    );

    Ok(())
}

fn parse_area(input: &str) -> Result<Area, String> {
    let xs = input.chars().filter(|c| *c == '\n').count();
    let ys = input.len() / xs - 1; // -1 input.len() also contains the line breaks
    if input.len() % xs != 0 {
        return Err("Area is not a rectangle".to_owned());
    }

    let cells: Vec<Cell> = input.chars().filter_map(|c| parse_cell(c)).collect();
    if cells.len() != xs * ys {
        return Err(format!(
            "Expected {} cells ({}×{}), but found {}.",
            xs * ys,
            xs,
            ys,
            cells.len()
        ));
    }
    return Ok(Area { xs, ys, cells });
}

fn parse_cell(c: char) -> Option<Cell> {
    match c {
        '.' => Some(Cell::Open),
        '|' => Some(Cell::Trees),
        '#' => Some(Cell::Lumberyard),
        _ => None,
    }
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
    fn after_generations_works_for_example() {
        // given
        let area_str = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.\n";
        let initial_area = parse_area(area_str).unwrap();

        // when
        let result = after_generations(&initial_area, 10);
        println!("Area after 10 minutes:\n{}", result);

        // then
        assert_eq!(result.count_cells(Cell::Trees), 37);
        assert_eq!(result.count_cells(Cell::Lumberyard), 31);
    }
}
