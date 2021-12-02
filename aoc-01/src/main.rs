// Advent of Code 2021 Day 01

use aoclib;
use std::io;

fn do_count(data: &[i32]) -> usize {
    let res: usize = data
    .windows(2)
    .filter(|d| d[1] > d[0])
    .count();

    return res;
}

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-01.txt")?;

    let prog_data = lines
    .iter()
    .map(|a| (a.parse::<i32>().unwrap()))
    .collect::<Vec<_>>();

    //println!("{:#?}", prog_data);

    let part1 = do_count(&prog_data);
    println!("Part 1: {}", part1);

    let triple_sum = prog_data
    .windows(3)
    .map(|a| a.iter().sum())
    .collect::<Vec<_>>();

    //println!("{:#?}", triple_sum);

    let part2 = do_count(&triple_sum);
    println!("Part 2: {}", part2);

    Ok(())
}
