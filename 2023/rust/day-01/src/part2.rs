use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input.lines().map(process_line).sum::<u32>();
    Ok(result.to_string())
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|i| {
        let substr = &line[i..];
        if substr.starts_with("one") {
            Some(1)
        } else if substr.starts_with("two") {
            Some(2)
        } else if substr.starts_with("three") {
            Some(3)
        } else if substr.starts_with("four") {
            Some(4)
        } else if substr.starts_with("five") {
            Some(5)
        } else if substr.starts_with("six") {
            Some(6)
        } else if substr.starts_with("seven") {
            Some(7)
        } else if substr.starts_with("eight") {
            Some(8)
        } else if substr.starts_with("nine") {
            Some(9)
        } else {
            substr.chars().next().and_then(|ch| ch.to_digit(10))
        }
    });
    let first = it.next().expect("should be a number");
    match it.last() {
        Some(last) => first * 10 + last,
        None => first * 10 + first,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("twone", 21)]
    #[case("eightwo", 82)]
    fn test_process_line(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(input))
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(
            (29 + 83 + 13 + 24 + 42 + 14 + 76).to_string(),
            process(input)?
        );
        Ok(())
    }
}
