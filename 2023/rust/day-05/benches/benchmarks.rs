use day_05::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1_binary_search() {
    part1_binary_search::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}

#[divan::bench]
fn part1_btree() {
    part1_btree::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}

#[divan::bench]
fn part2_binary_search() {
    part2_binary_search::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}
#[divan::bench]
fn part2_btree() {
    part2_btree::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}
