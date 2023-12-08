use std::collections::HashMap;

use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    optimized(input)
}

/// Optimized solution using LCM
fn optimized(input: &str) -> miette::Result<usize, AocError> {
    let (instructions, nodes) = parse_input(input);

    let result = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .map(|node| {
            let mut current = node;
            let mut step = 0;
            while !current.ends_with('Z') {
                current = match instructions[step % instructions.len()] {
                    'L' => nodes[current].0,
                    'R' => nodes[current].1,
                    _ => unreachable!(),
                };
                step += 1;
            }
            step
        })
        .fold(1, lcm);

    Ok(result)
}

/// least common multiple
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

/// greatest common divisor
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[allow(dead_code)]
/// Brute force solution - Takes too long to run
fn brute_force(input: &str) -> miette::Result<usize, AocError> {
    let (instructions, nodes) = parse_input(input);

    let mut step = 0;
    let mut current = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .collect::<Vec<_>>();
    while !current.iter().all(|k| k.ends_with('Z')) {
        current = current
            .iter()
            .map(|n| match instructions[step % instructions.len()] {
                'L' => nodes[n].0,
                'R' => nodes[n].1,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
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
    fn test_brute_force() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, brute_force(input)?);
        Ok(())
    }

    #[test]
    fn test_lcm() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, optimized(input)?);
        Ok(())
    }
}

