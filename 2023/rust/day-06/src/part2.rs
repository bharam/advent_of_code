use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let input = input
        .lines()
        .map(|line| {
            line.split(':')
                .last()
                .expect("should be numbers")
                .replace(' ', "")
                .parse::<usize>()
                .expect("should be a number")
        })
        .collect::<Vec<_>>();

    let time = input[0];
    let dist = input[1];

    let result = match binary_search(0, time / 2, time, dist) {
        Some(i) => {
            if time % 2 == 0 {
                ((time / 2) - i) * 2 - 1
            } else {
                ((time / 2) - i) * 2
            }
        }
        None => 0,
    };

    Ok(result)
}

/// Binary search for the largest value in the range that is <= the min_dist.
fn binary_search(mut lo: usize, mut hi: usize, time: usize, min_dist: usize) -> Option<usize> {
    assert!(lo < hi, "Range must be non-empty");

    let mut result = None;

    while lo < hi {
        let mid = (hi + lo) / 2;
        let dist = mid * (time - mid);
        if dist <= min_dist {
            lo = mid + 1;
            result = Some(usize::max(result.unwrap_or(usize::MIN), mid));
        } else {
            hi = mid;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(71503, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_input() -> miette::Result<()> {
        let input = "Time:        59     68     82     74
Distance:   543   1020   1664   1022";
        assert_eq!(37286485, process(input)?);
        Ok(())
    }
}

