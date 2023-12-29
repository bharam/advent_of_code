use itertools::Itertools;

use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let res = input
        .split("\n\n")
        .map(|pat| {
            let mat = pattern_to_matrix(pat);
            if let Some(i) = find_reflection(&mat) {
                i * 100
            } else {
                let mat = transpose(mat);
                find_reflection(&mat).expect("tranposed pattern must have reflection")
            }
        })
        .sum::<usize>();
    Ok(res)
}

fn pattern_to_matrix(pattern: &str) -> Vec<Vec<bool>> {
    pattern
        .lines()
        .map(|line| line.chars().map(|ch| ch == '#').collect_vec())
        .collect_vec()
}

fn find_reflection(mat: &[Vec<bool>]) -> Option<usize> {
    for (i, win) in mat.windows(2).enumerate() {
        if difference(&win[0], &win[1]) <= 1 {
            let (fst, lst) = mat.split_at(i + 1);
            if fst
                .iter()
                .rev()
                .zip(lst.iter())
                .map(|(a, b)| difference(a, b))
                .sum::<usize>()
                == 1
            {
                return Some(i + 1);
            }
        }
    }
    None
}

fn transpose(mat: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    assert!(!mat.is_empty());
    let (_, n) = (mat.len(), mat[0].len());
    (0..n)
        .map(|c| mat.iter().map(|r| r[c]).collect_vec())
        .collect_vec()
}

fn difference(a: &[bool], b: &[bool]) -> usize {
    a.iter().zip(b.iter()).filter(|(a, b)| a != b).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case(indoc!{
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#." }, 3)]
    #[case(indoc!{
        "#...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#" }, 1)]
    fn test_pattern(#[case] input: &str, #[case] expected: usize) {
        let input = pattern_to_matrix(input);
        if let Some(actual) = find_reflection(&input) {
            assert_eq!(expected, actual);
        } else {
            let input = transpose(input);
            assert_eq!(expected, find_reflection(&input).unwrap());
        }
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = indoc! {
            "#.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#"
        };
        assert_eq!(400, process(input)?);
        Ok(())
    }
}

