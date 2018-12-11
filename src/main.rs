static GRID_SIDE: i32 = 300;

fn main() -> Result<(), String> {
    let puzzle_input: i32 = 5177;

    let grid = power_grid(puzzle_input);
    let (xmax, ymax) = max_square(&grid);

    println!("Position of highest power: {}×{}", xmax, ymax);

    Ok(())
}

fn empty_grid() -> Vec<i32> {
    (0..(GRID_SIDE * GRID_SIDE)).map(|_| 0).collect()
}

fn power_grid(serial: i32) -> Vec<i32> {
    let mut grid = empty_grid();
    for y in 1..GRID_SIDE + 1 {
        for x in 1..(GRID_SIDE + 1) {
            grid[index(x, y)] = power_level(x, y, serial);
        }
    }
    return grid;
}

fn max_square(grid: &[i32]) -> (i32, i32) {
    let mut max: i32 = i32::min_value();
    let mut pos: (i32, i32) = (1, 1);
    for x in 1..GRID_SIDE - 1 {
        for y in 1..(GRID_SIDE - 1) {
            let power = grid[index(x, y)]
                + grid[index(x + 1, y)]
                + grid[index(x + 2, y)]
                + grid[index(x, y + 1)]
                + grid[index(x + 1, y + 1)]
                + grid[index(x + 2, y + 1)]
                + grid[index(x, y + 2)]
                + grid[index(x + 1, y + 2)]
                + grid[index(x + 2, y + 2)];
            if power > max {
                max = power;
                pos = (x, y);
            }
        }
    }
    return pos;
}

fn index(x: i32, y: i32) -> usize {
    ((x - 1) + (y - 1) * GRID_SIDE) as usize
}

fn power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let base_power = (y * rack_id + serial) * rack_id;
    (base_power % 1000) / 100 - 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn power_level_work_for_examples() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}
