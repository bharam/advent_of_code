use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let mut input = input.split("\n\n");
    assert_eq!(input.clone().count(), 8);
    let seeds = input
        .next()
        .expect("should be seeds")
        .split(": ")
        .last()
        .expect("should be seeds")
        .split_whitespace()
        .map(|n| n.parse::<u64>().expect("should be a number"))
        .collect::<Vec<_>>();

    let maps = input
        .map(|section| {
            let mut map: BTreeMap<u64, (u64, u64)> = BTreeMap::new(); // Key: source start, Value: (dest start, len)
            section.lines().skip(1).for_each(|line| {
                let nums = line
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().expect("should be a number"))
                    .collect::<Vec<_>>();
                map.insert(nums[1], (nums[0], nums[2]));
            });
            map
        })
        .collect::<Vec<_>>();

    let result = seeds
        .iter()
        .map(|seed| {
            let mut source = *seed;
            for map in maps.iter() {
                if let Some((&src_start, &(dst_start, len))) =
                    map.range((Unbounded, Included(&source))).next_back()
                {
                    if src_start <= source && source <= src_start + len {
                        source = dst_start + (source - src_start);
                    }
                }
            }
            source
        })
        .min()
        .unwrap();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(35, process(input)?);
        Ok(())
    }
}
