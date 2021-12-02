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
    .map(|(a,b)| (a, b.parse::<i32>().unwrap()))
    .collect::<Vec<(_,_)>>();

    //println!("{:#?}", prog_data);

    let mut hpos = 0i32;
    let mut depth = 0i32;
    for &(inst, x) in prog_data.iter() {
        match inst {
            "forward" => { hpos += x; }
            "up" => { depth -= x; }
            "down" => { depth += x; }
            _ => { println!("unexpected command"); }
        }
    }

    let part1 = hpos * depth;
    println!("Part 1: {}", part1);

    let mut hpos = 0i32;
    let mut aim = 0i32;
    let mut depth = 0i32;
    for &(inst, x) in prog_data.iter() {
        match inst {
            "forward" => { hpos += x; depth += aim * x; }
            "up" => { aim -= x; }
            "down" => { aim += x; }
            _ => { println!("unexpected command"); }
        }
    }

    let part2 = hpos * depth;
    println!("Part 2: {}", part2);

    Ok(())
}
