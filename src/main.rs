use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let mut number_input = content
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok());
    let tree = read_tree(&mut number_input)?;

    let metadata_sum = sum_metadata(&tree);
    println!("The sum of all metadata is {}", metadata_sum);

    let root_value = node_value(&tree);
    println!("The value of the root node is {}", root_value);

    Ok(())
}

fn sum_metadata(tree: &Node) -> usize {
    let metadata_sum: usize = tree.metadata.iter().sum();
    let children_metadata_sum: usize = tree.children.iter().map(|child| sum_metadata(child)).sum();

    return metadata_sum + children_metadata_sum;
}

fn node_value(node: &Node) -> usize {
    if node.children.len() == 0 {
        return node.metadata.iter().sum();
    }
    // Prof. Simon: "Rekursion kann töricht sein"
    let mut lookup_children: Vec<Option<usize>> = (0..node.children.len()).map(|_| None).collect();
    let mut sum: usize = 0;
    for index in node
        .metadata
        .iter()
        .filter(|i| **i <= node.children.len() && **i > 0)
        .map(|i| i - 1)
    {
        if let Some(child_value) = lookup_children[index] {
            sum += child_value;
        } else {
            let child_value = node_value(&node.children[index]);
            lookup_children[index] = Some(child_value);
            sum += child_value;
        }
    }
    return sum;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn read_tree(input: &mut Iterator<Item = usize>) -> Result<Node, String> {
    let n_children: usize = input
        .next()
        .ok_or_else(|| "Unexpected end of input, expected number of children".to_owned())?;
    let n_metadata: usize = input
        .next()
        .ok_or_else(|| "Unexpected end of input, expected meta data size".to_owned())?;
    let mut children = Vec::with_capacity(n_children);
    for _ in 0..n_children {
        children.push(read_tree(input)?);
    }
    let mut metadata = Vec::with_capacity(n_metadata);
    for _ in 0..n_metadata {
        let data = input
            .next()
            .ok_or_else(|| "Unexpected end of input, expected metadata".to_owned())?;
        metadata.push(data);
    }
    return Ok(Node { children, metadata });
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

    static EXAMPLE_INPUT: &[usize] = &[2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];

    #[test]
    fn read_tree_should_parse_example_tree() {
        // given
        let mut input = EXAMPLE_INPUT.iter().map(|u| *u);

        // when
        let tree = read_tree(&mut input).unwrap();

        // then
        assert_eq!(tree.metadata, vec![1, 1, 2]);
        assert_eq!(tree.children.len(), 2);

        let b = &tree.children[0];
        assert_eq!(b.metadata, vec![10, 11, 12]);
        assert_eq!(b.children.len(), 0);

        let c = &tree.children[1];
        assert_eq!(c.metadata, vec![2]);
        assert_eq!(c.children.len(), 1);

        let d = &c.children[0];
        assert_eq!(d.metadata, vec!(99));
        assert_eq!(d.children.len(), 0);
    }

    #[test]
    fn sum_metadata_should_work_for_example() {
        // given
        let mut input = EXAMPLE_INPUT.iter().map(|u| *u);
        let tree = read_tree(&mut input).unwrap();

        // when
        let sum = sum_metadata(&tree);

        // then
        assert_eq!(sum, 138);
    }

    #[test]
    fn node_value_should_work_for_example() {
        // given
        let mut input = EXAMPLE_INPUT.iter().map(|u| *u);
        let tree = read_tree(&mut input).unwrap();

        // when
        let value = node_value(&tree);

        // then
        assert_eq!(value, 66);
    }
}
