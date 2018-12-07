#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();

    let inverse_dag = parse_inverse_dag(&lines);
    let opt_ordered_nodes = order_nodes(&inverse_dag);

    if let Some(ordered_nodes) = opt_ordered_nodes {
        println!(
            "The correct order is: {}",
            ordered_nodes.iter().collect::<String>()
        );
    } else {
        println!("Apparently, the graph is not a fully connected DAG.");
    }

    Ok(())
}

fn order_nodes(inv_dag: &HashMap<char, Vec<char>>) -> Option<Vec<char>> {
    let mut result: Vec<char> = Vec::with_capacity(inv_dag.len());
    while result.len() < inv_dag.len() {
        let min_independent_node = inv_dag
            .iter()
            .filter(|(dependant, dependencies)| {
                !result.contains(dependant) && dependencies.iter().all(|dep| result.contains(&dep))
            })
            .min_by_key(|(dependant, _)| dependant.clone())?;
        result.push(*min_independent_node.0)
    }
    return Some(result);
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
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
            return edges;
        },
    )
}

fn parse_line(line: &str) -> Option<(char, char)> {
    lazy_static! {
        static ref RE_DEP: Regex =
            Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
    }
    let capture = RE_DEP.captures(line)?;
    let dependency = capture.get(1)?.as_str().chars().next()?;
    let dependant = capture.get(2)?.as_str().chars().next()?;

    return Some((dependency, dependant));
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
    fn order_nodes_works_for_example() {
        // given
        let dag = parse_inverse_dag(&EXAMPLE_LINES);

        // when
        let result = order_nodes(&dag);

        // then
        assert_eq!(result, Some(vec!['C', 'A', 'B', 'D', 'F', 'E']))
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
