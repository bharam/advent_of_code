use std::{cmp::Reverse, collections::BinaryHeap};

use crate::errors::AocError;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let mut grid = parse(input);
    grid = expand_universe(grid);
    let galaxies = get_galaxies(&grid);

    let mut result = 0;
    for (i, &galaxy1) in galaxies.iter().enumerate() {
        for &galaxy2 in galaxies.iter().skip(i + 1) {
            result += distance(galaxy1, galaxy2);
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

fn expand_universe(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
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
        .filter_map(|(i, x)| if !x { Some(i) } else { None });
    let cols_to_add = present_cols
        .iter()
        .enumerate()
        .filter_map(|(i, x)| if !x { Some(i) } else { None })
        .collect_vec();

    let mut new_grid = Vec::new();
    let mut row_heap = BinaryHeap::from_iter(
        grid.into_iter()
            .enumerate()
            .map(|(i, row)| (Reverse(i), row))
            .chain(rows_to_add.map(|i| (Reverse(i), vec!['.'; n]))),
    );
    while let Some((_, row)) = row_heap.pop() {
        let mut temp_row = Vec::new();
        let mut col_heap = BinaryHeap::from_iter(
            row.into_iter()
                .enumerate()
                .map(|(i, ch)| (Reverse(i), ch))
                .chain(cols_to_add.iter().map(|&i| (Reverse(i), '.'))),
        );
        while let Some((_, ch)) = col_heap.pop() {
            temp_row.push(ch);
        }
        new_grid.push(temp_row);
    }
    new_grid
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

fn distance(p: Coordinate, q: Coordinate) -> usize {
    p.0.abs_diff(q.0) + p.1.abs_diff(q.1)
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
    "}, indoc! {"
        ....#........
        .........#...
        #............
        .............
        .............
        ........#....
        .#...........
        ............#
        .............
        .............
        .........#...
        #....#......."}
    )]
    fn test_expand(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        let actual = expand_universe(parse(input))
            .iter()
            .map(|line| line.iter().collect::<String>())
            .join("\n");
        assert_eq!(expected, actual);
        Ok(())
    }
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
