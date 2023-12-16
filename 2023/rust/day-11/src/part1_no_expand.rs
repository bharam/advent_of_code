use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let grid = parse(input);
    let galaxies = get_galaxies(&grid);
    let (rows, cols) = to_expand(&grid);

    let mut result = 0;
    for (i, &galaxy1) in galaxies.iter().enumerate() {
        for &galaxy2 in galaxies.iter().skip(i + 1) {
            let mut dist = distance(galaxy1, galaxy2);
            let (xrange, yrange) = range(galaxy1, galaxy2);
            dist += rows.iter().filter(|r| xrange.contains(r)).count();
            dist += cols.iter().filter(|c| yrange.contains(c)).count();
            result += dist;
        }
    }
    Ok(result)
}

type Coordinate = (usize, usize);

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn get_galaxies(grid: &[Vec<char>]) -> Vec<Coordinate> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &space)| if space == '#' { Some((i, j)) } else { None })
        })
        .collect_vec()
}

fn to_expand(grid: &[Vec<char>]) -> (Vec<usize>, Vec<usize>) {
    let (m, n) = (grid.len(), grid[0].len());
    let mut present_rows = vec![false; m];
    let mut present_cols = vec![false; n];

    for i in 0..m {
        for (j, &ch) in grid[i].iter().enumerate() {
            if ch == '#' {
                present_rows[i] = true;
                present_cols[j] = true;
            }
        }
    }

    let rows_to_add = present_rows
        .iter()
        .enumerate()
        .filter_map(|(i, x)| if !x { Some(i) } else { None })
        .collect_vec();
    let cols_to_add = present_cols
        .iter()
        .enumerate()
        .filter_map(|(i, x)| if !x { Some(i) } else { None })
        .collect_vec();
    (rows_to_add, cols_to_add)
}

fn distance(p: Coordinate, q: Coordinate) -> usize {
    p.0.abs_diff(q.0) + p.1.abs_diff(q.1)
}

fn range(p: Coordinate, q: Coordinate) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let (xmin, xmax) = (p.0.min(q.0), p.0.max(q.0));
    let (ymin, ymax) = (p.1.min(q.1), p.1.max(q.1));
    (xmin..=xmax, ymin..=ymax)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case(indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "}, 374
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
