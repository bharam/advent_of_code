use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let grid = parse(input);
    let mut looop = find_loop(&grid);
    looop.push(looop[0]);

    Ok(shoelace_formula(&looop) as usize)
}

fn parse(input: &str) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| Pipe::new(ch, r, c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn find_loop(grid: &[Vec<Pipe>]) -> Vec<&Pipe> {
    let start = grid
        .iter()
        .find_map(|pipes| pipes.iter().find(|pipe| pipe.is_start()))
        .expect("Start pipe must exist");

    let mut next = start
        .get_neighbors(&grid, None)
        .next()
        .expect("Starting pipe must have at least one connected pipe");

    let mut path = vec![start];
    while next != start {
        let prev = *path.last().expect("Path must have at least one element");
        path.push(next);
        next = next
            .get_neighbors(grid, Some(prev))
            .next()
            .expect("Pipe must have at least one other neighbor than prev");
    }
    path
}

fn shoelace_formula(looop: &[&Pipe]) -> isize {
    let n = looop.len() as isize - 1;
    // 2A = area of polygon including the loop
    let twice_area = looop
        .windows(2)
        .map(|w| (w[0].row as isize * w[1].col as isize) - (w[0].col as isize * w[1].row as isize))
        .sum::<isize>()
        .abs();
    // subtract the area of the loop itself
    (twice_area - n) / 2 + 1
}

#[derive(Debug, Eq, PartialEq)]
struct Pipe {
    shape: char,
    row: usize,
    col: usize,
}

impl Pipe {
    fn new(shape: char, row: usize, col: usize) -> Self {
        Self { shape, row, col }
    }

    fn is_start(&self) -> bool {
        self.shape == 'S'
    }

    fn connected_with(&self, other: &Self) -> bool {
        let r_diff = self.row as isize - other.row as isize;
        let c_diff = self.col as isize - other.col as isize;
        static DOWN: [char; 4] = ['S', 'F', '7', '|'];
        static UP: [char; 4] = ['S', 'L', 'J', '|'];
        static LEFT: [char; 4] = ['S', 'J', '7', '-'];
        static RIGHT: [char; 4] = ['S', 'F', 'L', '-'];
        match (r_diff, c_diff) {
            (1, 0) => UP.contains(&self.shape) && DOWN.contains(&other.shape),
            (-1, 0) => DOWN.contains(&self.shape) && UP.contains(&other.shape),
            (0, 1) => LEFT.contains(&self.shape) && RIGHT.contains(&other.shape),
            (0, -1) => RIGHT.contains(&self.shape) && LEFT.contains(&other.shape),
            _ => unreachable!(),
        }
    }

    fn get_neighbors<'a>(
        &'a self,
        grid: &'a [Vec<Pipe>],
        previous: Option<&'a Pipe>,
    ) -> impl Iterator<Item = &Pipe> + 'a {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .filter_map(|(dr, dc)| {
                let (r, c) = (self.row as isize + dr, self.col as isize + dc);

                if 0 <= r && r < grid.len() as isize && 0 <= c && c < grid[0].len() as isize {
                    Some(&grid[r as usize][c as usize])
                } else {
                    None
                }
            })
            .filter(move |pipe| {
                pipe.shape != '.' && Some(*pipe) != previous && self.connected_with(pipe)
            })
    }
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) - {}", self.row, self.col, self.shape)
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        4
    )]

    #[case(
        "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
        4
    )]

#[case(
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
8
)]

    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        10
    )]

    fn test_process(#[case] input: &str, #[case] expected: usize) -> miette::Result<()> {
        // unimplemented!();
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
