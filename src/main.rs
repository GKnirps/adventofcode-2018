use std::collections::HashMap;

fn geological_index(
    x: u64,
    y: u64,
    lookup_map: &mut HashMap<(u64, u64), u64>,
    target_x: u64,
    target_y: u64,
) -> u64 {
    if x == 0 && y == 0 {
        return 0;
    }
    if x == target_x && y == target_y {
        return 0;
    }
    if y == 0 {
        return x * 16807;
    }
    if x == 0 {
        return y * 48271;
    }
    if let Some(gi) = lookup_map.get(&(x, y)) {
        return *gi;
    }
    let left = geological_index(x - 1, y, lookup_map, target_x, target_y);
    let up = geological_index(x, y - 1, lookup_map, target_x, target_y);
    let gi = left * up;
    lookup_map.insert((x, y), gi);
    return gi;
}

static EROSION_MOD: u64 = 20183;

fn erosion_level(
    x: u64,
    y: u64,
    geo_map: &mut HashMap<(u64, u64), u64>,
    target_x: u64,
    target_y: u64,
    depth: u64,
) -> u64 {
    let gi = geological_index(x, y, geo_map, target_x, target_y);
    return (gi + depth) % EROSION_MOD;
}

fn risk_level(depth: u64, target_x: u64, target_y: u64) -> u64 {
    let mut geo_map: HashMap<(u64, u64), u64> =
        HashMap::with_capacity((target_x * target_y) as usize);
    let mut risk_level = 0;
    for x in 0..(target_x + 1) {
        for y in 0..(target_y + 1) {
            risk_level += erosion_level(x, y, &mut geo_map, target_x, target_y, depth) % 3;
        }
    }
    return risk_level;
}

fn main() -> Result<(), String> {
    // this stuff is specific for my input, but I did'nt want to bother with parsing
    // just two lines
    let depth: u64 = 8112;
    let (target_x, target_y): (u64, u64) = (13, 743);
    // input end

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn risk_level_works_for_example() {
        // given
        let depth: u64 = 510;
        let tx: u64 = 10;
        let ty: u64 = 10;

        // when
        let risk = risk_level(depth, tx, ty);

        // then
        assert_eq!(risk, 114);
    }
}
