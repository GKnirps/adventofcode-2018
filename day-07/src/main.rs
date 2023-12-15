use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();

    let inverse_dag = parse_inverse_dag(&lines);
    let opt_ordered_nodes = work_on_nodes(&inverse_dag, 1);

    if let Some(ordered_nodes) = opt_ordered_nodes {
        println!(
            "The correct order is: {}",
            ordered_nodes.0.iter().collect::<String>()
        );
    } else {
        println!("Apparently, the graph is not a fully connected DAG.");
    }

    let opt_parallel_result = work_on_nodes(&inverse_dag, 5);

    if let Some(parallel_result) = opt_parallel_result {
        println!("With four helping elves, it takes {} seconds. The steps have been finished in order {}", parallel_result.1, parallel_result.0.iter().collect::<String>());
    } else {
        println!("With four helping elves, everything ended in chaos.");
    }

    Ok(())
}

fn time(node: char) -> u32 {
    61 + (node.to_ascii_uppercase() as u32 - 'A' as u32)
}

fn work_on_nodes(inv_dag: &HashMap<char, Vec<char>>, n_workers: usize) -> Option<(Vec<char>, u32)> {
    let mut result: Vec<char> = Vec::with_capacity(inv_dag.len());
    let mut workers: Vec<Option<(char, u32)>> = (0..n_workers).map(|_| None).collect();
    let mut used_time: u32 = 0;

    while result.len() < inv_dag.len() {
        // give available jobs to free workers
        for i in 0..workers.len() {
            if workers[i].is_none() {
                if let Some(min_independent_node) = get_next_node(inv_dag, &result, &workers) {
                    workers[i] = Some((min_independent_node, time(min_independent_node)));
                } else {
                    break;
                }
            }
        }
        // minimum time to wait until at least one worker finishes
        let wait_time = workers.iter().filter_map(|w| w.map(|(_, t)| t)).min()?;

        // update required time
        workers = workers
            .iter()
            .map(|worker| worker.map(|(node, t)| (node, t - wait_time)))
            .collect();
        used_time += wait_time;

        // mark finished jobs
        for (node, _) in workers.iter().filter_map(|w| *w).filter(|(_, t)| *t == 0) {
            result.push(node);
        }
        // remove finished jobs from workers
        workers = workers
            .iter()
            .map(|w| w.and_then(|(node, time)| if time == 0 { None } else { Some((node, time)) }))
            .collect();
    }

    Some((result, used_time))
}

fn get_next_node(
    inv_dag: &HashMap<char, Vec<char>>,
    finished_jobs: &[char],
    workers: &[Option<(char, u32)>],
) -> Option<char> {
    inv_dag
        .iter()
        .filter(|(dependant, dependencies)| {
            !finished_jobs.contains(dependant)
                && !workers
                    .iter()
                    .filter_map(|w| *w)
                    .any(|(node, _)| node == **dependant)
                && dependencies.iter().all(|dep| finished_jobs.contains(dep))
        })
        .min_by_key(|(dependant, _)| *dependant)
        .map(|(d, _)| *d)
}

fn parse_inverse_dag(lines: &[&str]) -> HashMap<char, Vec<char>> {
    lines.iter().filter_map(|line| parse_line(line)).fold(
        HashMap::with_capacity(lines.len()),
        |mut edges, (dependency, dependant)| {
            edges
                .entry(dependant)
                .or_insert_with(|| Vec::with_capacity(10))
                .push(dependency);
            // just so we have all edges in the map, we also insert the dependency node
            edges
                .entry(dependency)
                .or_insert_with(|| Vec::with_capacity(10));
            edges
        },
    )
}

fn parse_line(line: &str) -> Option<(char, char)> {
    let (dependency, dependant) = line.split_once(" must be finished before step ")?;
    let dependency = dependency.strip_prefix("Step ")?.chars().next()?;
    let dependant = dependant.strip_suffix(" can begin.")?.chars().next()?;

    Some((dependency, dependant))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_LINES: [&str; 7] = [
        "Step C must be finished before step A can begin.",
        "Step C must be finished before step F can begin.",
        "Step A must be finished before step B can begin.",
        "Step A must be finished before step D can begin.",
        "Step B must be finished before step E can begin.",
        "Step D must be finished before step E can begin.",
        "Step F must be finished before step E can begin.",
    ];

    #[test]
    fn work_on_nodes_works_for_example() {
        // given
        let dag = parse_inverse_dag(&EXAMPLE_LINES);

        // when
        let result = work_on_nodes(&dag, 1).expect("expected a result");

        // then
        assert_eq!(result.0, vec!['C', 'A', 'B', 'D', 'F', 'E'])
    }

    #[test]
    fn parse_inverse_dags_works_correctly() {
        // when
        let dag = parse_inverse_dag(&EXAMPLE_LINES);

        // then
        assert_eq!(dag.len(), 6);
        assert_eq!(dag.get(&'C'), Some(&vec![]));
        assert_eq!(dag.get(&'A'), Some(&vec!['C']));
        assert_eq!(dag.get(&'F'), Some(&vec!['C']));
        assert_eq!(dag.get(&'B'), Some(&vec!['A']));
        assert_eq!(dag.get(&'D'), Some(&vec!['A']));
        assert_eq!(dag.get(&'E'), Some(&vec!['B', 'D', 'F']));
    }

    #[test]
    fn parse_line_parses_dependency_correctly() {
        // given
        let line = "Step D must be finished before step E can begin.";

        // when
        let (dependency, dependant) = parse_line(line).expect("Expected something");

        // then
        assert_eq!(dependency, 'D');
        assert_eq!(dependant, 'E');
    }
}
