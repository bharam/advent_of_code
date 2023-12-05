use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let mut input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (m, n) = (input.len(), input[0].len());

    let mut result = 0;
    for r in 0..m {
        for c in 0..n {
            if input[r][c] == '*' {
                let adj_numbers = find_adj_numbers(&mut input, r, c);
                if adj_numbers.len() == 2 {
                    result += adj_numbers.iter().product::<u32>();
                }
            }
        }
    }

    Ok(result)
}

/// Returns the adjacent numbers of the given position
fn find_adj_numbers(mat: &mut Vec<Vec<char>>, row: usize, col: usize) -> Vec<u32> {
    let mut result = Vec::new();
    for r in row.saturating_sub(1)..=row.saturating_add(1) {
        for c in col.saturating_sub(1)..=col.saturating_add(1) {
            if r < mat.len() && c < mat[0].len() {
                if mat[r][c].is_digit(10) {
                    result.push(get_number(mat, r, c));
                }
            }
        }
    }
    result
}

/// Returns the number
fn get_number(mat: &mut Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut number = mat[row][col].to_digit(10).unwrap();
    let (mut start, mut end) = (col, col);
    let mut multiplier = 10;
    while let Some(i) = start.checked_sub(1) {
        if let Some(n) = mat[row][i].to_digit(10) {
            number = n * multiplier + number;
            mat[row][i] = '.';
            start = i;
            multiplier *= 10;
        } else {
            break;
        }
    }
    while let Some(i) = end.checked_add(1) {
        if i > mat[0].len() - 1 {
            break;
        }
        if let Some(n) = mat[row][i].to_digit(10) {
            number = number * 10 + n;
            mat[row][i] = '.';
            end = i;
        } else {
            break;
        }
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, process(input)?);
        Ok(())
    }
}
