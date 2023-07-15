use aoc_8;
use std::fs;

fn main() {
    part_1();
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = aoc_8::process_part_1(&input);
    println!("Part1: {}", result);
}
