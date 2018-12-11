static GRID_SIDE: i32 = 300;

fn main() -> Result<(), String> {
    let puzzle_input: i32 = 5177;

    let grid = power_grid(puzzle_input);
    let cumsum = cumsum_grid(&grid);
    let (xmax, ymax) = max_3_square(&cumsum);

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

fn cumsum_grid(grid: &[i32]) -> Vec<i32> {
    let mut cumsum = grid.to_vec();
    for y in 1..GRID_SIDE {
        for x in 1..GRID_SIDE {
            let index = (x + y * GRID_SIDE) as usize;
            let index_left = (x - 1 + y * GRID_SIDE) as usize;
            let index_upper = (x + (y - 1) * GRID_SIDE) as usize;
            let index_remove = (x - 1 + (y - 1) * GRID_SIDE) as usize;
            cumsum[index] =
                cumsum[index] + cumsum[index_left] + cumsum[index_upper] - cumsum[index_remove];
        }
    }
    return cumsum;
}

fn max_3_square(cumsum: &[i32]) -> (i32, i32) {
    let mut max: i32 = i32::min_value();
    let mut pos: (i32, i32) = (1, 1);
    for x in 1..GRID_SIDE - 1 {
        for y in 1..(GRID_SIDE - 1) {
            let power = area_value(cumsum, x, y, 2, 2);
            if power > max {
                max = power;
                pos = (x, y);
            }
        }
    }
    return pos;
}

fn area_value(cumsum: &[i32], x: i32, y: i32, xs: i32, ys: i32) -> i32 {
    let mut value = cumsum[index(x + xs, y + ys)];
    if x > 1 {
        value -= cumsum[index(x - 1, y + ys)];
    }
    if y > 1 {
        value -= cumsum[index(x + xs, y - 1)];
    }
    if x > 1 && y > 1 {
        value += cumsum[index(x - 1, y - 1)];
    }
    return value;
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
