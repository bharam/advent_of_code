use day_11::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// #[divan::bench]
// fn part1_expand_bfs() {
//     part1_expand_bfs::process(divan::black_box(include_str!("../input.txt",))).unwrap();
// }

#[divan::bench]
fn part1_expand() {
    part1_expand::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}

#[divan::bench]
fn part1_no_expand() {
    part1_no_expand::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}

#[divan::bench]
fn part1_no_expand_opt() {
    part1_no_expand_opt::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../input.txt",)), 1e6 as usize).unwrap();
}
