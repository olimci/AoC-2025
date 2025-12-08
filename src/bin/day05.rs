use std::fs;
use std::cmp;

#[derive(Debug)]
struct Range {
    low:  usize,
    high: usize,
}

impl Range {
    fn new(low: usize, high: usize) -> Range {
        Range { low: low, high: high }
    }

    fn contains(&self, v: usize) -> bool {
        self.low <= v && v <= self.high
    }

    fn size(&self) -> usize {
        self.high - self.low + 1
    }
}

#[derive(Debug)]
struct RangeSet {
    ranges: Vec<Range>,
}

impl RangeSet {
    fn new() -> RangeSet {
        RangeSet { ranges: Vec::new() }
    }

    fn insert(&mut self, mut new: Range) {
        let mut i = 0;
        while i < self.ranges.len() {
            let r = &self.ranges[i];

            if r.high + 1 < new.low {
                i += 1;
            } else if new.high + 1 < r.low {
                break;
            } else {
                new.low  = cmp::min(new.low,  r.low);
                new.high = cmp::max(new.high, r.high);
                self.ranges.remove(i);
            }
        }

        self.ranges.insert(i, new);
    }

    fn contains(&self, v: usize) -> bool {
        for range in self.ranges.iter() {
            if range.contains(v) {
                return true
            }
        }

        return false
    }

    fn size(&self) -> usize {
        self.ranges.iter().map(|r| r.size()).sum()
    }
}

fn solve_1(input: &str) {
    let (input_ranges, input_ids) = input.split_once("\n\n").unwrap();

    let mut ranges = RangeSet::new();
    let mut fresh = 0;

    for line in input_ranges.lines() {
        let (ls, hs) = line.split_once('-').unwrap();
        let low: usize = ls.parse().unwrap();
        let high: usize = hs.parse().unwrap();

        ranges.insert(Range::new(low, high));
    }

    let total = ranges.size();

    for line in input_ids.lines() {
        let id: usize = line.parse().unwrap();

        if ranges.contains(id) {
            fresh += 1;
        }
    }

    println!("Fresh: {fresh}");
    println!("Total: {total}");
}

fn main() {
    let input = fs::read_to_string("input/day05.txt")
        .expect("failed to read file!");

    solve_1(&input);
}

