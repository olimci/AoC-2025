use std::fs;

use aoc_2025::util;

fn part_1() -> isize {
    let input = fs::read_to_string("input/day01.txt")
        .expect("failed to read file!");

    let mut counter = 50;
    let mut zero_counter = 0;

    for line in input.lines() {
        let dir = &line[0..1];

        let num: isize = line[1..]
            .parse()
            .unwrap_or_else(|e| panic!("failed to parse {line}: {e}"));

        match dir {
            "L" => counter -= num,
            "R" => counter += num,
            _   => panic!("unknown direction {dir}!"),
        }

        counter = util::modulo(counter, 100);

        if counter == 0 {
            zero_counter += 1;
        }
    }

    zero_counter
}

fn part_2() -> isize {
    let input = fs::read_to_string("input/day01.txt")
        .expect("failed to read file!");

    let mut counter = 50;
    let mut zero_counter = 0;

    for line in input.lines() {
        let dir = &line[0..1];

        let num: isize = line[1..]
            .parse()
            .unwrap_or_else(|e| panic!("failed to parse line {line}: {e}"));

        let sign = match dir {
            "L" => -1,
            "R" => 1,
            _   => panic!("unknown direction {dir}!"),
        };

        let raw = counter + sign * num;
        let next = util::modulo(raw, 100);
        let wraps = raw.div_euclid(100).abs()
            + if next == 0    && sign == -1 {1} else {0}
            - if counter == 0 && sign == -1 {1} else {0};


        println!("{} + {} -> {} -> {} ({})", counter, num*sign, raw, next, wraps);

        zero_counter += wraps;
        counter = next;
    }

    zero_counter
}

fn main() {
    let part_1_answer = part_1();
    println!("Part 1 answer:\n{part_1_answer}");

    println!("\n---\n");

    let part_2_answer = part_2();
    println!("Part 2 answer:\n{part_2_answer}")
}