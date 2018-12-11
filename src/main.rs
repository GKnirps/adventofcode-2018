fn main() -> Result<(), String> {
    let puzzle_input: i32 = 5177;

    Ok(())
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
