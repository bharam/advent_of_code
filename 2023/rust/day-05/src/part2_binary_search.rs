use crate::errors::AocError;

#[derive(Debug, PartialEq)]
struct Range {
    src: u64,
    dst: u64,
    len: u64,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.src.partial_cmp(&other.src)
    }
}

impl Range {
    fn new(src: u64, dst: u64, len: u64) -> Self {
        Self { src, dst, len }
    }

    fn contains(&self, target: u64) -> bool {
        self.src <= target && target < self.src + self.len
    }

    fn get(&self, target: u64) -> u64 {
        assert!(
            self.contains(target),
            "target {} not in range {:?}",
            target,
            self
        );
        self.dst + (target - self.src)
    }
}

#[derive(Debug)]
struct Map(Vec<Range>);

impl Map {
    fn new(map: Vec<Range>) -> Self {
        Self(map)
    }

    fn get(&self, target: u64) -> u64 {
        let mut lo = 0;
        let mut hi = self.0.len();

        while lo < hi {
            let mid = (lo + hi) / 2;
            let range = &self.0[mid];
            if range.contains(target) {
                return range.get(target);
            } else if target < range.src {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        target
    }
}

#[derive(Debug)]
struct Maps(Vec<Map>);
impl Maps {
    fn new(maps: Vec<Map>) -> Self {
        Self(maps)
    }

    fn get(&self, target: u64) -> u64 {
        self.0.iter().fold(target, |target, map| map.get(target))
    }

    /// * `lo`: start of the seed range (inclusive)
    /// * `hi`: end of the seed range (inclusive)
    fn get_range(&self, lo: u64, hi: u64) -> u64 {
        assert!(lo <= hi, "Seed range must be non-empty");
        if hi == lo {
            return self.get(lo);
        }

        let lo_val = self.get(lo);
        let hi_val = self.get(hi);

        if lo_val < hi_val && hi_val - lo_val == hi - lo {
            // the destination range is continuous
            lo_val
        } else {
            let mid = (hi + lo) / 2;
            u64::min(self.get_range(lo, mid), self.get_range(mid + 1, hi))
        }
    }
}

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
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<_>>();

    let maps = Maps::new(
        input
            .map(|section| {
                let mut map = section
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let nums = line
                            .split_whitespace()
                            .map(|n| n.parse::<u64>().expect("should be a number"))
                            .collect::<Vec<_>>();
                        Range::new(nums[1], nums[0], nums[2])
                    })
                    .collect::<Vec<_>>();
                map.sort_unstable_by_key(|range| range.src);
                Map::new(map)
            })
            .collect::<Vec<_>>(),
    );

    let result = seeds
        .iter()
        .map(|&(seed, len)| maps.get_range(seed, seed + len - 1))
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
        assert_eq!(46, process(input)?);
        Ok(())
    }
}
