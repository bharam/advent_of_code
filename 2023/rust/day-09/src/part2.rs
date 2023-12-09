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

fn predict_history(history: Vec<i32>) -> i32 {
    let mut sequences = vec![history];
    let mut curr_seq = sequences.last().unwrap();
    while !curr_seq.iter().all(|&n| n == 0) {
        let next_seq = curr_seq
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>();
        sequences.push(next_seq);
        curr_seq = &sequences.last().unwrap();
    }

    let result = sequences
        .into_iter()
        .rev()
        .skip(1)
        .map(|seq| *seq.first().unwrap())
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

