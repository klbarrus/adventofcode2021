// Advent of Code 2021 Day 02

use aoclib;
use itertools::Itertools;
use std::io;

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-02.txt")?;

    let prog_data = lines
        .iter()
        .flat_map(|l| l.split(" "))
        .tuples()
        .map(|(a, b)| (a, b.parse::<i32>().unwrap()))
        .collect::<Vec<(_, _)>>();

    //println!("{:#?}", prog_data);

    let (hpos, depth) = prog_data
        .iter()
        .fold((0, 0), |(h, d), (op, x)| match op {
            &"forward" => (h + x, d),
            &"up" => (h, d - x),
            &"down" => (h, d + x),
        _ => (h, d),
    });

    let part1 = hpos * depth;
    println!("Part 1: {}", part1);

    let (hpos, depth, _) = prog_data
        .iter()
        .fold((0, 0, 0), |(h, d, a), (op, x)| match op {
            &"forward" => (h + x, d + a * x, a),
            &"up" => (h, d, a - x),
            &"down" => (h, d, a + x),
            _ => (h, d, a),
        });

    let part2 = hpos * depth;
    println!("Part 2: {}", part2);

    Ok(())
}
