use itertools::Itertools;

use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let mat = parse_input(input);
    let res = total_load(mat);
    Ok(res)
}

fn total_load(mat: Vec<Vec<char>>) -> usize {
    mat.iter().map(row_load).sum::<usize>()
}

fn row_load(row: &Vec<char>) -> usize {
    let n = row.len();
    let mut ptr = 0_usize;
    row.iter()
        .enumerate()
        .map(|(j, ch)| match ch {
            '#' => {
                ptr = j + 1;
                0
            }
            'O' => {
                let load = n - ptr;
                ptr += 1;
                load
            }
            '.' => 0,
            _ => unreachable!(),
        })
        .sum::<usize>()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mat = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    transpose(mat)
}

fn transpose(mat: Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!mat.is_empty());
    let (_, n) = (mat.len(), mat[0].len());
    (0..n)
        .map(|c| mat.iter().map(|r| r[c]).collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case(indoc!{"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
        "}, 136)]
    fn test_process(#[case] input: &str, #[case] expected: usize) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
