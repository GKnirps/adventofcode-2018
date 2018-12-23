use std::collections::{hash_map::Entry, HashMap};

static EROSION_MOD: u64 = 20183;

fn geologic_index(
    x: u64,
    y: u64,
    lookup_map: &mut HashMap<(u64, u64), u64>,
    target_x: u64,
    target_y: u64,
    depth: u64,
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
    let left = erosion_level(x - 1, y, lookup_map, target_x, target_y, depth);
    let up = erosion_level(x, y - 1, lookup_map, target_x, target_y, depth);
    let gi = left * up;
    lookup_map.insert((x, y), gi);
    return gi;
}

fn erosion_level(
    x: u64,
    y: u64,
    geo_map: &mut HashMap<(u64, u64), u64>,
    target_x: u64,
    target_y: u64,
    depth: u64,
) -> u64 {
    let gi = geologic_index(x, y, geo_map, target_x, target_y, depth);
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Equipment {
    Climbing,
    Torch,
    Neither,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    px: u64,
    py: u64,
    equipment: Equipment,
}

fn equ_allowed_for_type(terrain: u64, equipment: Equipment) -> bool {
    match terrain {
        // rocky
        0 => match equipment {
            Equipment::Neither => false,
            _ => true,
        },
        // wet
        1 => match equipment {
            Equipment::Torch => false,
            _ => true,
        },
        // narrow
        2 => match equipment {
            Equipment::Climbing => false,
            _ => true,
        },
        // invalid terrain type
        _ => false,
    }
}

fn other_equ_for_type(terrain: u64, equipment: Equipment) -> Equipment {
    match terrain {
        // rock
        0 => {
            if equipment == Equipment::Torch {
                Equipment::Climbing
            } else {
                Equipment::Torch
            }
        }
        // wet
        1 => {
            if equipment == Equipment::Climbing {
                Equipment::Neither
            } else {
                Equipment::Climbing
            }
        }
        // narrow
        2 => {
            if equipment == Equipment::Torch {
                Equipment::Neither
            } else {
                Equipment::Torch
            }
        }
        // invalid terrain type
        _ => panic!("invalid terrain type"),
    }
}

// find all neighbours of a node, and return them along with the distance from the given node
// important: a neighbour can also have the same position, but different equipment
fn get_neighbours(
    node: &Node,
    target_x: u64,
    target_y: u64,
    depth: u64,
    geo_map: &mut HashMap<(u64, u64), u64>,
) -> Vec<(Node, u64)> {
    let mut neighbours: Vec<(Node, u64)> = Vec::with_capacity(5);
    if node.px > 0
        && equ_allowed_for_type(
            erosion_level(node.px - 1, node.py, geo_map, target_x, target_y, depth) % 3,
            node.equipment,
        ) {
        neighbours.push((
            Node {
                px: node.px - 1,
                py: node.py,
                equipment: node.equipment,
            },
            1,
        ));
    }
    if node.py > 0
        && equ_allowed_for_type(
            erosion_level(node.px, node.py - 1, geo_map, target_x, target_y, depth) % 3,
            node.equipment,
        ) {
        neighbours.push((
            Node {
                px: node.px,
                py: node.py - 1,
                equipment: node.equipment,
            },
            1,
        ));
    }
    if equ_allowed_for_type(
        erosion_level(node.px + 1, node.py, geo_map, target_x, target_y, depth) % 3,
        node.equipment,
    ) {
        neighbours.push((
            Node {
                px: node.px + 1,
                py: node.py,
                equipment: node.equipment,
            },
            1,
        ));
    }
    if equ_allowed_for_type(
        erosion_level(node.px, node.py + 1, geo_map, target_x, target_y, depth) % 3,
        node.equipment,
    ) {
        neighbours.push((
            Node {
                px: node.px,
                py: node.py + 1,
                equipment: node.equipment,
            },
            1,
        ));
    }
    let other_equ = other_equ_for_type(
        erosion_level(node.px, node.py, geo_map, target_x, target_y, depth) % 3,
        node.equipment,
    );
    neighbours.push((
        Node {
            px: node.px,
            py: node.py,
            equipment: other_equ,
        },
        7,
    ));
    return neighbours;
}

fn shortest_path(target_x: u64, target_y: u64, depth: u64) -> Option<u64> {
    let mut geo_map: HashMap<(u64, u64), u64> =
        HashMap::with_capacity((target_x * target_y) as usize);
    // map of visited nodes (with distance in minutes)
    let mut visited: HashMap<Node, u64> =
        HashMap::with_capacity((target_x * target_y * 2) as usize);
    // reachable nodes with the minimal distance to reach them
    // yeah, I know this is not the most efficient way to search for the closest neighbour, but I
    // hope it works
    let mut reachable: HashMap<Node, u64> =
        HashMap::with_capacity((target_x * target_y * 2) as usize);

    let source_node = Node {
        px: 0,
        py: 0,
        equipment: Equipment::Torch,
    };
    for (node, dist) in get_neighbours(&source_node, target_x, target_y, depth, &mut geo_map) {
        reachable.insert(node, dist);
    }

    visited.insert(source_node, 0);
    let target_node = Node {
        px: target_x,
        py: target_y,
        equipment: Equipment::Torch,
    };

    while let Some((next_node, dist)) = reachable
        .iter()
        .min_by_key(|(_, d)| *d)
        .map(|(n, d)| (n.clone(), *d))
    {
        if next_node == target_node {
            // target found, stop here
            return Some(dist);
        }
        reachable.remove(&next_node);

        // update reachable nodes
        for (neighbour, rel_dist) in
            get_neighbours(&next_node, target_x, target_y, depth, &mut geo_map)
        {
            if visited.contains_key(&neighbour) {
                continue;
            }
            let neigh_dist = dist + rel_dist;
            match reachable.entry(neighbour) {
                Entry::Vacant(v) => {
                    v.insert(neigh_dist);
                }
                Entry::Occupied(mut o) => {
                    if neigh_dist < *o.get() {
                        o.insert(neigh_dist);
                    }
                }
            }
        }

        // add node to visited nodes
        visited.insert(next_node, dist);
    }
    // we cannot visit any more nodes but haven't found the target yet. Sound like someone
    // isn't going to be rescued.
    // Of course, given the problem specification, this cannot happen (but may anyways if we implemented the algorithm wrong)
    return None;
}

fn print_area(depth: u64, xmax: u64, ymax: u64) {
    let mut geo_map: HashMap<(u64, u64), u64> = HashMap::with_capacity((xmax * xmax) as usize);
    for y in 0..(ymax + 1) {
        for x in 0..(xmax + 1) {
            match erosion_level(x, y, &mut geo_map, xmax, ymax, depth) % 3 {
                0 => {
                    print!(".");
                }
                1 => {
                    print!("=");
                }
                2 => {
                    print!("|");
                }
                _ => {
                    print!("\\033[1;31mE\\033[0m");
                }
            };
        }
        println!("");
    }
}

fn main() -> Result<(), String> {
    // this stuff is specific for my input, but I did'nt want to bother with parsing
    // just two lines
    let depth: u64 = 8112;
    let (target_x, target_y): (u64, u64) = (13, 743);
    // input end

    let ri = risk_level(depth, target_x, target_y);
    println!("Risk level of the area is {}", ri);

    let distance = shortest_path(target_x, target_y, depth).expect("Expected a valid path");
    println!("The fastest path takes {} minutes.", distance);

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
        print_area(depth, tx, ty);
        assert_eq!(risk, 114);
    }

    #[test]
    fn shortest_path_works_for_example() {
        // given
        let depth: u64 = 510;
        let tx: u64 = 10;
        let ty: u64 = 10;

        // when
        let distance = shortest_path(tx, ty, depth).expect("Expected a path");

        // then
        assert_eq!(distance, 45);
    }
}
