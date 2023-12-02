use once_cell::sync::Lazy;
use regex::Regex;

use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let mut result = 0;
    for line in input.lines() {
        let (max_red, max_green, max_blue) = process_line(line)?;
        result += max_red * max_green * max_blue;
    }
    Ok(result)
}

pub fn process_line(line: &str) -> miette::Result<(u32, u32, u32), AocError> {
    let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?P<num>\d+) (?P<color>red|green|blue)").unwrap());
    for cap in RE.captures_iter(line) {
        let num = cap["num"].parse::<u32>()?;
        match &cap["color"] {
            "red" => max_red = max_red.max(num),
            "green" => max_green = max_green.max(num),
            "blue" => max_blue = max_blue.max(num),
            _ => unreachable!(),
        }
    }
    Ok((max_red, max_green, max_blue))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", (4, 2, 6))]
    #[case("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", (1, 3, 4))]
    #[case("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", (20, 13, 6))]
    #[case("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", (14, 3, 15))]
    #[case("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", (6, 3, 2))]
    fn test_process_line(
        #[case] input: &str,
        #[case] expected: (u32, u32, u32),
    ) -> miette::Result<()> {
        assert_eq!(expected, process_line(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
