use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input
        .lines()
        .map(|line| {
            let first = line.chars().find_map(|ch| ch.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|ch| ch.to_digit(10)).unwrap();
            first * 10 + last
        })
        .sum::<u32>()
        .to_string();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1abc3", "13")]
    #[case("13", "13")]
    #[case("arst1tnoien3orsiet", "13")]
    #[case("5", "55")]
    fn test_process_line(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
