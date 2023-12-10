use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32, AocError> {
    let input = parse_input(input);

    let result = input.into_iter().map(predict_history).sum::<i32>();

    Ok(result)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().expect("should be a number"))
                .collect()
        })
        .collect()
}

fn predict_history(mut history: Vec<i32>) -> i32 {
    let mut evidences = vec![history.first().copied().unwrap()];
    while !history.iter().all(|&n| n == 0) {
        history = history
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>();
        evidences.push(history.first().copied().unwrap());
    }

    let result = evidences
        .into_iter()
        .rev()
        .skip(1)
        .reduce(|acc, n| n - acc)
        .unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, process(input)?);
        Ok(())
    }
}
