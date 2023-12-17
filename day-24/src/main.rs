use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (immune_system, infection) = parse(&content)?;

    if let Some((remaining_victors, _)) = fight(immune_system.clone(), infection.clone()) {
        println!("The remaining party has {remaining_victors} left.");
    } else {
        println!("The fighting has come to a stalemate.");
    }

    let minimal_boosted_victors = find_minimal_required_boost(&immune_system, &infection);
    println!(
        "With a minimal winning boost, the immune system has {minimal_boosted_victors} units left"
    );

    Ok(())
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Group<'a> {
    number: u32,
    hit_points: u32,
    weak: Vec<&'a str>,
    immune: Vec<&'a str>,
    attack: u32,
    damage_type: &'a str,
    initiative: u32,
}

impl Group<'_> {
    fn effective_power(&self) -> u32 {
        self.number * self.attack
    }

    fn cmp_effective_initiative(&self, other: &Self) -> Ordering {
        self.effective_power()
            .cmp(&other.effective_power())
            .then(self.initiative.cmp(&other.initiative))
    }

    fn damage_to(&self, other: &Self) -> u32 {
        self.effective_power()
            * if other.immune.contains(&self.damage_type) {
                0
            } else if other.weak.contains(&self.damage_type) {
                2
            } else {
                1
            }
    }
}

fn fight(mut immune_system: Vec<Group>, mut infection: Vec<Group>) -> Option<(u32, bool)> {
    let mut targets_immune_system: Vec<Option<usize>> = Vec::with_capacity(immune_system.len());
    let mut targets_infection: Vec<Option<usize>> = Vec::with_capacity(infection.len());
    let mut targeted_infection: HashSet<usize> = HashSet::with_capacity(infection.len());
    let mut targeted_immune_system: HashSet<usize> = HashSet::with_capacity(immune_system.len());
    let mut attack_order_immune: Vec<(usize, u32)> = Vec::with_capacity(immune_system.len());
    let mut attack_order_infection: Vec<(usize, u32)> = Vec::with_capacity(infection.len());

    let mut total_units: u32 = immune_system.iter().map(|group| group.number).sum::<u32>()
        + infection.iter().map(|group| group.number).sum::<u32>();
    while !immune_system.is_empty() && !infection.is_empty() {
        targets_immune_system.clear();
        targets_infection.clear();
        targeted_infection.clear();
        targeted_immune_system.clear();
        attack_order_immune.clear();
        attack_order_infection.clear();

        // target selection phase
        immune_system.sort_unstable_by(|g1, g2| g2.cmp_effective_initiative(g1));
        infection.sort_unstable_by(|g1, g2| g2.cmp_effective_initiative(g1));
        for immune_group in &immune_system {
            let target_i = infection
                .iter()
                .enumerate()
                .filter(|(i, inf_group)| {
                    !targeted_infection.contains(i) && immune_group.damage_to(inf_group) > 0
                })
                .max_by(|(_, inf_group_1), (_, inf_group_2)| {
                    immune_group
                        .damage_to(inf_group_1)
                        .cmp(&immune_group.damage_to(inf_group_2))
                        .then_with(|| {
                            inf_group_1
                                .effective_power()
                                .cmp(&inf_group_2.effective_power())
                        })
                        .then(inf_group_1.initiative.cmp(&inf_group_2.initiative))
                })
                .map(|(i, _)| i);
            targets_immune_system.push(target_i);
            if let Some(i) = target_i {
                targeted_infection.insert(i);
            }
        }
        for inf_group in &infection {
            let target_i = immune_system
                .iter()
                .enumerate()
                .filter(|(i, immune_group)| {
                    !targeted_immune_system.contains(i) && inf_group.damage_to(immune_group) > 0
                })
                .max_by(|(_, immune_group_1), (_, immune_group_2)| {
                    inf_group
                        .damage_to(immune_group_1)
                        .cmp(&inf_group.damage_to(immune_group_2))
                        .then_with(|| {
                            immune_group_1
                                .effective_power()
                                .cmp(&immune_group_2.effective_power())
                        })
                        .then(immune_group_1.initiative.cmp(&immune_group_2.initiative))
                })
                .map(|(i, _)| i);
            targets_infection.push(target_i);
            if let Some(i) = target_i {
                targeted_immune_system.insert(i);
            }
        }
        // attacking phase
        attack_order_immune.extend(
            immune_system
                .iter()
                .enumerate()
                .map(|(i, group)| (i, group.initiative)),
        );
        attack_order_immune.sort_unstable_by(|(_, ini1), (_, ini2)| ini2.cmp(ini1));
        attack_order_infection.extend(
            infection
                .iter()
                .enumerate()
                .map(|(i, group)| (i, group.initiative)),
        );
        attack_order_infection.sort_unstable_by(|(_, ini1), (_, ini2)| ini2.cmp(ini1));
        let mut immune_attack_i: usize = 0;
        let mut infection_attack_i: usize = 0;
        while immune_attack_i < attack_order_immune.len()
            || infection_attack_i < attack_order_infection.len()
        {
            if attack_order_immune
                .get(immune_attack_i)
                .map(|(_, immune_ini)| {
                    attack_order_infection
                        .get(infection_attack_i)
                        .map(|(_, infection_ini)| immune_ini.cmp(infection_ini))
                        .unwrap_or(Ordering::Greater)
                })
                .unwrap_or(Ordering::Less)
                .is_gt()
            {
                let immune_i = attack_order_immune[immune_attack_i].0;
                let attack_group = &immune_system[immune_i];
                if let Some(target_i) = targets_immune_system[immune_i] {
                    let target_group = &mut infection[target_i];
                    target_group.number = target_group.number.saturating_sub(
                        attack_group.damage_to(target_group) / target_group.hit_points,
                    );
                }
                immune_attack_i += 1;
            } else {
                let infection_i = attack_order_infection[infection_attack_i].0;
                let attack_group = &infection[infection_i];
                if let Some(target_i) = targets_infection[infection_i] {
                    let target_group = &mut immune_system[target_i];
                    target_group.number = target_group.number.saturating_sub(
                        attack_group.damage_to(target_group) / target_group.hit_points,
                    );
                }
                infection_attack_i += 1;
            }
        }
        immune_system.retain(|group| group.number > 0);
        infection.retain(|group| group.number > 0);
        let new_total_units: u32 = immune_system.iter().map(|group| group.number).sum::<u32>()
            + infection.iter().map(|group| group.number).sum::<u32>();
        // if no units are killed in a round, no units will be killed in the future
        //  => infinite loop. This is a stalemate and has now winners
        if new_total_units == total_units {
            return None;
        }
        total_units = new_total_units;
    }
    Some((total_units, !immune_system.is_empty()))
}

fn fight_boosted(
    mut immune_system: Vec<Group>,
    infection: Vec<Group>,
    boost: u32,
) -> Option<(u32, bool)> {
    for group in &mut immune_system {
        group.attack += boost;
    }
    fight(immune_system, infection)
}

fn find_minimal_required_boost(immune_system: &[Group], infection: &[Group]) -> u32 {
    // assumption: a boost _is_ required
    let mut lower: u32 = 0;
    let mut upper: u32 = 16;

    // first: find any boost that makes the immune system win
    let mut outcome = fight_boosted(immune_system.to_vec(), infection.to_vec(), upper);
    while !outcome.map(|(_, won)| won).unwrap_or(false) {
        lower = upper;
        upper *= 2;
        outcome = fight_boosted(immune_system.to_vec(), infection.to_vec(), upper);
    }
    let mut upper_units_left = outcome.unwrap().0;

    // then do a binary search to find the minimal required boost
    while upper != lower + 1 {
        let pivot = lower + (upper - lower) / 2;
        let outcome = fight_boosted(immune_system.to_vec(), infection.to_vec(), pivot);
        if let Some((units_left, won)) = outcome {
            if won {
                upper_units_left = units_left;
                upper = pivot;
            } else {
                lower = pivot;
            }
        } else {
            lower = pivot;
        }
    }
    upper_units_left
}

fn parse(input: &str) -> Result<(Vec<Group>, Vec<Group>), String> {
    let (immune_system, infection) = input
        .split_once("\n\nInfection:\n")
        .ok_or_else(|| "unable to split immune system from infection".to_string())?;
    let immune_system: Vec<Group> = immune_system
        .strip_prefix("Immune System:\n")
        .ok_or_else(|| "missing header for immune system".to_string())?
        .lines()
        .map(parse_group)
        .collect::<Result<_, _>>()?;
    let infection: Vec<Group> = infection
        .lines()
        .map(parse_group)
        .collect::<Result<_, _>>()?;

    Ok((immune_system, infection))
}

fn parse_group(line: &str) -> Result<Group, String> {
    let (number, rest) = line
        .split_once(" units each with ")
        .ok_or_else(|| format!("unable to split unit number from rest in line '{line}'"))?;
    let number: u32 = number
        .parse()
        .map_err(|e| format!("unable to parse number of units '{number}': {e}"))?;

    let (hit_points, rest) = rest
        .split_once(" hit points ")
        .ok_or_else(|| format!("unable to split hit points from rest in line '{line}'"))?;
    let hit_points: u32 = hit_points
        .parse()
        .map_err(|e| format!("unable to parse hit poiunts '{hit_points}': {e}"))?;

    let (weak, immune, rest) = if let Some(rest) = rest.strip_prefix('(') {
        let (mods, rest) = rest.split_once(") ").ok_or_else(|| {
            "unable to split rest from weaknesses and immunities '{rest}'".to_string()
        })?;
        let (weak, immune) =
            mods.split("; ")
                .try_fold((vec![], vec![]), |(mut weak, mut immune), statement| {
                    if let Some(list) = statement.strip_prefix("weak to ") {
                        weak.extend(list.split(", "));
                    } else if let Some(list) = statement.strip_prefix("immune to ") {
                        immune.extend(list.split(", "));
                    } else {
                        return Err(format!("expected weakness or immunity, found '{statement}"));
                    }
                    Ok((weak, immune))
                })?;
        (weak, immune, rest)
    } else {
        (vec![], vec![], rest)
    };

    let (attack, rest) = rest
        .strip_prefix("with an attack that does ")
        .ok_or_else(|| format!("unable to find attack damage in '{rest}'"))?
        .split_once(' ')
        .ok_or_else(|| {
            format!("unable to find separator between attack damage and damage type in '{rest}'")
        })?;
    let attack: u32 = attack
        .parse()
        .map_err(|e| format!("unable to parse attack damage '{attack}': {e}"))?;

    let (damage_type, initiative) = rest
        .split_once(" damage at initiative ")
        .ok_or_else(|| format!("unable to separate damage type from initiative in '{rest}'"))?;
    let initiative: u32 = initiative
        .parse()
        .map_err(|e| format!("unable to parse initiative '{initiative}': {e}"))?;
    Ok(Group {
        number,
        hit_points,
        weak,
        immune,
        attack,
        damage_type,
        initiative,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
"#;

    #[test]
    fn fight_works_for_example() {
        // given
        let (immune_system, infection) = parse(EXAMPLE).expect("expected successful parsing");

        // when
        let leftovers = fight(immune_system, infection);

        // then
        assert_eq!(leftovers, Some((5216, false)));
    }

    #[test]
    fn find_minimal_required_boost_works_for_example() {
        // given
        let (immune_system, infection) = parse(EXAMPLE).expect("expected successful parsing");

        // when
        let leftovers = find_minimal_required_boost(&immune_system, &infection);

        // then
        assert_eq!(leftovers, 51);
    }
}
