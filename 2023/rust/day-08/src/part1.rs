use std::collections::HashMap;

use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (instructions, nodes) = parse_input(input);

    let mut step = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        current = match instructions[step % instructions.len()] {
            'L' => nodes[current].0,
            'R' => nodes[current].1,
            _ => unreachable!(),
        };
        step += 1;
    }

    Ok(step)
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().collect::<Vec<_>>();
    let nodes = nodes
        .lines()
        .map(parse_node)
        .collect::<HashMap<&str, (&str, &str)>>();
    (instructions, nodes)
}

fn parse_node(line: &str) -> (&str, (&str, &str)) {
    let (node, direction) = line.split_once(" = ").unwrap();
    let direction = direction
        .trim_matches(|c| c == '(' || c == ')')
        .split_once(", ")
        .unwrap();
    (node, direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
