use std::collections::HashMap;

use itertools::Itertools;

use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let mut mat = parse_input(input);

    let mut cache: HashMap<String, usize> = HashMap::new();
    let mut cycle_start = 0_usize;
    let mut cycle_len = 0_usize;

    for i in 0..1_000_000_000 {
        let encoded = encode(&mat);
        let memo = cache.entry(encoded).or_insert(i);
        if *memo != i {
            cycle_start = *memo;
            cycle_len = i - cycle_start;
            break;
        }
        spin(&mut mat);
    }

    let num_to_spin = (1_000_000_000 - cycle_start) % cycle_len;
    for _ in 0..num_to_spin {
        spin(&mut mat);
    }
    let res = calc_load(&mat);
    Ok(res)
}

fn encode(mat: &[Vec<char>]) -> String {
    mat.iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n")
}

fn calc_load(mat: &[Vec<char>]) -> usize {
    let m = mat.len();
    mat.iter()
        .enumerate()
        .map(|(i, row)| {
            let count = row.iter().filter(|ch| **ch == 'O').count();
            count * (m - i)
        })
        .sum::<usize>()
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn spin(mat: &mut Vec<Vec<char>>) {
    tilt_to(mat, Direction::North);
    tilt_to(mat, Direction::West);
    tilt_to(mat, Direction::South);
    tilt_to(mat, Direction::East);
}

fn tilt_to(mat: &mut Vec<Vec<char>>, direction: Direction) {
    match direction {
        Direction::North => {
            transposed(mat);
            tilt(mat);
            transposed(mat);
        }
        Direction::West => {
            tilt(mat);
        }
        Direction::South => {
            transposed(mat);
            reversed(mat);
            tilt(mat);
            reversed(mat);
            transposed(mat);
        }
        Direction::East => {
            reversed(mat);
            tilt(mat);
            reversed(mat);
        }
    }
}

fn tilt(mat: &mut Vec<Vec<char>>) {
    assert!(!mat.is_empty());
    let n = mat[0].len();
    mat.iter_mut().for_each(|row| {
        let mut ptr = 0_usize;
        (0..n).for_each(|j| match row[j] {
            '#' => {
                ptr = j + 1;
            }
            'O' => {
                row.swap(ptr, j);
                ptr += 1;
            }
            '.' => {}
            _ => unreachable!(),
        })
    })
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn transposed(mat: &mut Vec<Vec<char>>) {
    assert!(!mat.is_empty());
    let (_, n) = (mat.len(), mat[0].len());
    *mat = (0..n)
        .map(|c| mat.iter().map(|r| r[c]).collect_vec())
        .collect_vec();
}

fn reversed(mat: &mut Vec<Vec<char>>) {
    *mat = mat
        .iter()
        .map(|row| row.iter().copied().rev().collect_vec())
        .collect_vec();
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
        "}, 64)]
    fn test_process(#[case] input: &str, #[case] expected: usize) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }

    #[test]
    fn test_tilt_to() -> miette::Result<()> {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."};
        let mut mat = parse_input(input);
        tilt_to(&mut mat, Direction::North);
        let north_actual = mat
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        let north_expected = indoc! {"
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#...."};
        assert_eq!(north_expected, north_actual);

        tilt_to(&mut mat, Direction::West);
        let west_actual = mat
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        let west_expected = indoc! {"
            OOOO.#O...
            OO..#....#
            OOO..##O..
            O..#OO....
            ........#.
            ..#....#.#
            O....#OO..
            O.........
            #....###..
            #....#...."};
        assert_eq!(west_expected, west_actual);

        tilt_to(&mut mat, Direction::South);
        let south_actual = mat
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        let south_expected = indoc! {"
            .....#....
            ....#.O..#
            O..O.##...
            O.O#......
            O.O....O#.
            O.#..O.#.#
            O....#....
            OO....OO..
            #O...###..
            #O..O#...."};
        assert_eq!(south_expected, south_actual);

        tilt_to(&mut mat, Direction::East);
        let east_actual = mat
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        let east_expected = indoc! {"
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#...."};
        assert_eq!(east_expected, east_actual);

        Ok(())
    }

    #[test]
    fn test_spin() -> miette::Result<()> {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."};
        let mut mat = parse_input(input);
        spin(&mut mat);
        let actual = mat
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        let cicle_1 = indoc! {"
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#...."};
        assert_eq!(cicle_1, actual);

        spin(&mut mat);
        let actual = mat
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        let cycle_2 = indoc! {".....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O"};
        assert_eq!(cycle_2, actual);

        spin(&mut mat);
        let actual = mat
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        let cycle_3 = indoc! {".....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O"};
        assert_eq!(cycle_3, actual);

        Ok(())
    }

    /*
        OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....

        OOOO.#O...
        OO..#....#
        OOO..##O..
        O..#OO....
        ........#.
        ..#....#.#
        O....#OO..
        O.........
        #....###..
        #....#....

        .....#....
        ....#.O..#
        O..O.##...
        O.O#......
        O.O....O#.
        O.#..O.#.#
        O....#....
        OO....OO..
        #O...###..
        #O..O#....

        .....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#....

    */
}

