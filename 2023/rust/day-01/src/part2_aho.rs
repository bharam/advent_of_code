use aho_corasick::AhoCorasick;

use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input.lines().map(process_line).sum::<u32>();
    Ok(result.to_string())
}

fn process_line(line: &str) -> u32 {
    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let values = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    let ac = AhoCorasick::new(patterns).expect("should be able to build AhoCorasick");
    let mut matches = ac
        .find_overlapping_iter(line)
        .map(|m| values[m.pattern().as_usize() % 9]);
    let first = matches.next().expect("should be a number");
    let last = matches.last().unwrap_or(first);

    first * 10 + last
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
    #[case("1eightwo", 12)]
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
