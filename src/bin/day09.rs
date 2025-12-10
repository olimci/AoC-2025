use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    s: Point,
    e: Point,
}

impl Rect {
    fn new(s: Point, e: Point) -> Self {
        Rect { s, e }
    }

    fn area(&self) -> isize {
        ((self.e.x - self.s.x).abs() + 1) * ((self.e.y - self.s.y).abs() + 1)
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(",").collect();
            Point::new(parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

fn solve_1(input: &str) {
    let positions = parse_input(input);

    let mut max_area = 0;

    for (i, ip) in positions.iter().enumerate() {
        for (j, jp) in positions.iter().enumerate() {
            if i >= j {
                continue;
            }

            let area = Rect::new(ip.clone(), jp.clone()).area();

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Solution 1: {max_area}");
}

fn solve_2(input: &str) {
    let vertices = parse_input(input);

    let xs = vertices
        .iter()
        .map(|p| p.x)
        .unique()
        .sorted_unstable()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<_, _>>();

    let ys = vertices
        .iter()
        .map(|p| p.y)
        .unique()
        .sorted_unstable()
        .enumerate()
        .map(|(i, y)| (y, i))
        .collect::<HashMap<_, _>>();

    // get width / height from maximum keys of x/y
    let width = *xs.values().max().unwrap() + 1;
    let height = *ys.values().max().unwrap() + 1;

    let mut edges = vec![vec![false; width]; height];
    let mut grid = vec![vec![false; width]; height];

    // fill in edges with true if a vertical edge is in that space
    for i in 0..vertices.len() {
        let f = vertices[i];
        let t = vertices[(i + 1) % vertices.len()];

        if f.x == t.x {
            for i in ys[&f.y.min(t.y)]..ys[&f.y.max(t.y)] {
                edges[i][xs[&f.x]] = true;
            }
        }
    }

    for i in 0..height {
        let mut v = false;
        for j in 0..width {
            if edges[i][j] {
                grid[i][j] = true;
                v = !v;
            }

            if v {
                grid[i][j] = true;
            }
        }
    }

    let mut max_area = 0;

    for (i, ip) in vertices.iter().enumerate() {
        'outer: for (j, jp) in vertices.iter().enumerate() {
            if i >= j {
                continue;
            }

            let area = Rect::new(ip.clone(), jp.clone()).area();

            if area < max_area {
                continue 'outer;
            }

            for i in ys[&ip.y.min(jp.y)]..ys[&ip.y.max(jp.y)] {
                for j in xs[&ip.x.min(jp.x)]..xs[&ip.x.max(jp.x)] {
                    if !grid[i][j] {
                        continue 'outer;
                    }
                }
            }

            max_area = area;
        }
    }

    println!("Solution 2: {}", max_area);
}

fn main() {
    let input = fs::read_to_string("input/day09.txt").expect("failed to read file!");

    solve_1(&input);
    solve_2(&input);
}
