use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let input = input
        .lines()
        .map(|line| {
            let mut line = line.chars().collect::<Vec<_>>();
            line.push('.');
            line
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    let mut current = 0;
    let (mut start, mut end) = (None, None);
    for (r, row) in input.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if let Some(num) = ch.to_digit(10) {
                if start.is_none() {
                    start = Some(c);
                }
                end = Some(c);
                current = current * 10 + num;
            } else if current > 0 {
                if adj_symbol(&input, r, start.unwrap(), end.unwrap()) {
                    result += current;
                }
                current = 0;
                start = None;
                end = None;
            }
        }
    }

    Ok(result)
}

fn adj_symbol(mat: &Vec<Vec<char>>, row: usize, start: usize, end: usize) -> bool {
    for r in row.saturating_sub(1)..=row.saturating_add(1) {
        for c in start.saturating_sub(1)..=end.saturating_add(1) {
            if r < mat.len() && c < mat[0].len() {
                let ch = mat[r][c];
                if !ch.is_digit(10) && ch != '.' {
                    return true;
                }
            }
        }
    }
    false
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
        assert_eq!(4361, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "...............307............130..................969...601...186.........................................312....628..........878..........
......479#../..*..............................#.....*......*............../309.....484........................*......-..........+.....89....
...........726..352...461..69..............435.....390...625....................................459.........152...-....580............*.....
";
        assert_eq!(7252, process(input)?);
        Ok(())
    }
}
