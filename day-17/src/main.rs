use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.lines().collect();
    let veins = parse_veins(&lines);
    let (area, x_offset) =
        Area::from_veins(&veins).ok_or_else(|| "Unable to create area from veins".to_owned())?;

    let source_x = 500 - x_offset;
    let filled_area = fill_area(area, source_x, 0);
    let water_count = filled_area.count_water();
    println!(
        "There are {} tiles that can be reached by water",
        water_count
    );
    let stagnant_count = filled_area.count_stagnant_water();
    println!(
        "There are {} tiles of water that will not flow away after the source dries out",
        stagnant_count
    );

    Ok(())
}

fn fill_area(mut area: Area, source_x: usize, source_y: usize) -> Area {
    // don't go further if the flow is outside the observed area
    if source_x >= area.xs || source_y >= area.ys {
        return area;
    }
    // if there is anything but sand in this tile, there is no space for water here. Stop now.
    if area.tile(source_x, source_y) != Tile::Sand {
        return area;
    }
    // fill out the area below (if there is no sand below, this will do nothing)
    area = fill_area(area, source_x, source_y + 1);

    // go left until you hit a wall or a flow is below after filling what is below.
    // Remember if there is a wall on that side (so we can decide whether we have flow or stagnant water here
    let mut wall_left: bool = false;
    let mut x = source_x;
    let mut left_max = x;
    let mut more = area.tile(x, source_y + 1).is_full();
    while more && x > 0 {
        x -= 1;
        if area.tile(x, source_y) == Tile::Clay {
            wall_left = true;
            more = false;
            left_max = x + 1;
        } else {
            area = fill_area(area, x, source_y + 1);
            if !area.tile(x, source_y + 1).is_full() {
                left_max = x;
                more = false;
            }
        }
    }
    // now do the whole thing again in the other direction
    let mut wall_right: bool = false;
    let mut x = source_x;
    let mut right_max = x;
    let mut more = area.tile(x, source_y + 1).is_full();
    while more {
        x += 1;
        if area.tile(x, source_y) == Tile::Clay {
            wall_right = true;
            more = false;
            right_max = x - 1;
        } else {
            area = fill_area(area, x, source_y + 1);
            if !area.tile(x, source_y + 1).is_full() {
                more = false;
                right_max = x;
            }
        }
    }
    // now we just need to fill this row with an appropriate water tile
    let fill_tile = if wall_right && wall_left {
        Tile::Stagnant
    } else {
        Tile::Flow
    };

    for x in left_max..(right_max + 1) {
        area.set_tile(x, source_y, fill_tile);
    }

    area
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Sand,
    Clay,
    Flow,
    Stagnant,
}

impl Tile {
    fn is_water(self) -> bool {
        self == Tile::Flow || self == Tile::Stagnant
    }
    fn is_full(self) -> bool {
        self == Tile::Clay || self == Tile::Stagnant
    }
}

struct Area {
    xs: usize,
    ys: usize,
    tiles: Vec<Tile>,
}

impl Area {
    fn from_veins(veins: &[Vein]) -> Option<(Area, usize)> {
        // offset -1, because we need an additional empty column
        let x_offset: usize = veins.iter().map(|v| v.lower_x()).min()? - 1;
        let x_max: usize = veins.iter().map(|v| v.upper_x()).max()?;
        let y_offset: usize = veins.iter().map(|v| v.lower_y()).min()?;
        let y_max: usize = veins.iter().map(|v| v.upper_y()).max()?;

        // x size is +3 instead of +1, because we need an empty column left and right for overflow
        let xs = x_max - x_offset + 3;
        let ys = y_max - y_offset + 1;

        let mut tiles: Vec<Tile> = (0..(xs * ys)).map(|_| Tile::Sand).collect();
        for vein in veins {
            match vein {
                Vein::Horizontal { x1, x2, y } => {
                    for x in (*x1)..(x2 + 1) {
                        tiles[(x - x_offset) + (y - y_offset) * xs] = Tile::Clay;
                    }
                }
                Vein::Vertical { x, y1, y2 } => {
                    for y in (*y1)..(y2 + 1) {
                        tiles[(x - x_offset) + (y - y_offset) * xs] = Tile::Clay;
                    }
                }
            }
        }

        Some((Area { xs, ys, tiles }, x_offset))
    }

    fn tile(&self, x: usize, y: usize) -> Tile {
        *self.tiles.get(x + y * self.xs).unwrap_or(&Tile::Sand)
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.xs && y < self.ys {
            self.tiles[x + y * self.xs] = tile;
        }
    }

    fn count_water(&self) -> usize {
        self.tiles.iter().filter(|t| t.is_water()).count()
    }

    fn count_stagnant_water(&self) -> usize {
        self.tiles.iter().filter(|t| **t == Tile::Stagnant).count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Vein {
    Horizontal { x1: usize, x2: usize, y: usize },
    Vertical { x: usize, y1: usize, y2: usize },
}

impl Vein {
    fn upper_x(&self) -> usize {
        match self {
            Vein::Horizontal { x1, x2, y: _ } => *x1.max(x2),
            Vein::Vertical { x, y1: _, y2: _ } => *x,
        }
    }
    fn lower_x(&self) -> usize {
        match self {
            Vein::Horizontal { x1, x2, y: _ } => *x1.min(x2),
            Vein::Vertical { x, y1: _, y2: _ } => *x,
        }
    }
    fn upper_y(&self) -> usize {
        match self {
            Vein::Horizontal { x1: _, x2: _, y } => *y,
            Vein::Vertical { x: _, y1, y2 } => *y1.max(y2),
        }
    }
    fn lower_y(&self) -> usize {
        match self {
            Vein::Horizontal { x1: _, x2: _, y } => *y,
            Vein::Vertical { x: _, y1, y2 } => *y1.min(y2),
        }
    }
}

fn parse_veins(lines: &[&str]) -> Vec<Vein> {
    lines.iter().filter_map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Option<Vein> {
    let (start, range) = line.split_once(", ")?;
    let (orientation, d1) = start.split_once('=')?;
    let d1: usize = d1.parse().ok()?;
    let (_, range) = range.split_once('=')?;
    let (d2_1, d2_2) = range.split_once("..")?;
    let d2_1 = d2_1.parse().ok()?;
    let d2_2 = d2_2.parse().ok()?;
    match orientation {
        "x" => Some(Vein::Vertical {
            x: d1,
            y1: d2_1,
            y2: d2_2,
        }),
        "y" => Some(Vein::Horizontal {
            x1: d2_1,
            x2: d2_2,
            y: d1,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fill_area_works_for_example() {
        // given
        let lines = &[
            "x=495, y=2..7",
            "y=7, x=495..501",
            "x=501, y=3..7",
            "x=498, y=2..4",
            "x=506, y=1..2",
            "x=498, y=10..13",
            "x=504, y=10..13",
            "y=13, x=498..504",
        ];
        let veins = parse_veins(lines);
        let (area, x_offset) = Area::from_veins(&veins).expect("Expected valid area");

        // when
        let result = fill_area(area, 500 - x_offset, 0);
        let water_count = result.count_water();
        let stagnant_count = result.count_stagnant_water();

        // then
        assert_eq!(water_count, 57);
        assert_eq!(stagnant_count, 29);
    }

    #[test]
    fn parse_line_works_correctly() {
        assert_eq!(
            parse_line("x=569, y=570..582"),
            Some(Vein::Vertical {
                x: 569,
                y1: 570,
                y2: 582
            })
        );
        assert_eq!(
            parse_line("y=372, x=495..519"),
            Some(Vein::Horizontal {
                x1: 495,
                x2: 519,
                y: 372
            })
        );
    }
}
