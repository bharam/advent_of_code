use crate::errors::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let grid = parse(input);
    let looop = find_loop(&grid);

    Ok(looop.len() / 2)
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
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        4
    )]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        4
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        8
    )]
    #[case(
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        8
    )]
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
        23
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
