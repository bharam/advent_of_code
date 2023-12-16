use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
};

use crate::errors::AocError;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let mut grid = parse(input);
    grid = expand_universe(grid);
    let galaxies = get_galaxies(&grid);

    let mut found_pairs = HashSet::new();
    let result = galaxies
        .iter()
        .map(|&(i, j)| sum_distance_to_others(&grid, &mut found_pairs, i, j))
        .sum::<usize>();
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

fn sum_distance_to_others(
    grid: &[Vec<char>],
    found_pairs: &mut HashSet<(Coordinate, Coordinate)>,
    i: usize,
    j: usize,
) -> usize {
    let mut q = VecDeque::from_iter(get_neighbors(grid, i, j).map(|(i, j)| (i, j, 1)));
    let mut visited = HashSet::from([(i, j)]);

    let mut distance = 0_usize;
    while let Some((ci, cj, dist)) = q.pop_front() {
        if visited.contains(&(ci, cj)) {
            continue;
        }
        visited.insert((ci, cj));
        if grid[ci][cj] == '#' {
            let key = ((i, j).min((ci, cj)), (i, j).max((ci, cj)));
            if !found_pairs.contains(&key) {
                distance += dist;
                found_pairs.insert(key);
            }
        }
        for (ni, nj) in get_neighbors(grid, ci, cj) {
            q.push_back((ni, nj, dist + 1));
        }
    }
    distance
}

/// get positions of neighboring cells in left, right, or below. (no above neighbor)
fn get_neighbors(grid: &[Vec<char>], i: usize, j: usize) -> impl Iterator<Item = Coordinate> {
    assert!(!grid.is_empty() && !grid[0].is_empty());
    let (m, n) = (grid.len() as isize, grid[0].len() as isize);
    let (i, j) = (i as isize, j as isize);
    [(0, 1), (1, 0), (0, -1)]
        .iter()
        .filter_map(move |(di, dj)| {
            let (ni, nj) = (i + di, j + dj);
            if 0 <= ni && ni < m && 0 <= nj && nj < n {
                Some((ni as usize, nj as usize))
            } else {
                None
            }
        })
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
