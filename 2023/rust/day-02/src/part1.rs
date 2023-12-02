use once_cell::sync::Lazy;
use regex::Regex;

use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let mut result = 0;
    for line in input.lines() {
        let game_id = get_game_id(line)?;
        if process_line(line)? {
            result += game_id;
        }
    }
    Ok(result)
}

fn get_game_id(line: &str) -> miette::Result<u32, AocError> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (?P<id>\d+):\s?").unwrap());
    let cap = RE.captures(line).unwrap();
    Ok(cap["id"].parse::<u32>()?)
}

fn process_line(line: &str) -> miette::Result<bool, AocError> {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?P<num>\d+) (?P<color>red|green|blue)").unwrap());
    let (red, green, blue) = (12, 13, 14);
    for cap in RE.captures_iter(line) {
        let num = cap["num"].parse::<u32>()?;
        if (&cap["color"] == "red" && num > red)
            || (&cap["color"] == "green" && num > green)
            || (&cap["color"] == "blue" && num > blue)
        {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 2)]
    #[case(
        "Game 55: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        55
    )]
    fn test_get_game_id(#[case] input: &str, #[case] expected: u32) -> miette::Result<()> {
        assert_eq!(expected, get_game_id(input)?);
        Ok(())
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn test_process_line(#[case] input: &str, #[case] expected: bool) -> miette::Result<()> {
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
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
