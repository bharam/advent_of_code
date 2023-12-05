#![allow(dead_code, unused)]

use std::collections::HashSet;

use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let lines = input.lines().collect::<Vec<&str>>();
    let num_games = lines.len();
    let mut counter = vec![1; num_games];

    for (i, line) in lines.iter().enumerate() {
        let num_sets = line
            .split(':')
            .last()
            .expect("line should start with `Game #:`")
            .split('|')
            .map(|nums| {
                nums.split_whitespace()
                    .map(|n| n.parse::<u32>().expect("should be a number"))
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();
        let num_won = num_sets[0].intersection(&num_sets[1]).count();
        for j in 1..=num_won {
            if i + j < num_games {
                counter[i + j] += counter[i];
            }
        }
    }

    let result = counter.iter().sum::<u32>();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, process(input)?);
        Ok(())
    }
}
