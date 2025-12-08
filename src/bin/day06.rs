use std::fs;
use std::cmp;
use regex::Regex;

fn make_table(input: &str) -> (Vec<Vec<&str>>, usize, usize) {
    let re = Regex::new(r"(\d+|[+*])").unwrap();
    let table: Vec<Vec<&str>> = input
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str())
                .collect()
        })
        .filter(|row: &Vec<&str>| row.len() != 0)
        .collect();

    assert!(!table.is_empty(), "table has no rows!");

    let cols = table[0].len();
    assert!(table.iter().all(|row| row.len() == cols));
    let rows = table.len();

    (table, rows, cols)
}

fn transpose(s: &str) -> String {
    let mut grid: Vec<Vec<char>> = s
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    assert!(!grid.is_empty());

    let rows = grid.len();
    let mut cols = 0;

    for row in 0..rows {
        cols = cmp::max(cols, grid[row].len())
    }
    for row in 0..rows {
        let extra = cols - grid[row].len();
        for _ in 0..extra {
            grid[row].push(' ');
        }
    }

    assert!(grid.iter().all(|row| row.len() == cols));

    let mut r = String::new();

    for col in 0..cols {
        if col > 0 {
            r.push('\n');
        }
        for row in 0..rows {
            r.push(grid[row][col]);
        }
    }

    r
}

fn solve_1(input: &str) {
    let (table, rows, cols) = make_table(input);

    let mut total = 0;
    for col in 0..cols {
        let op = table[rows-1][col];
        let mut acc = match op {
            "+" => 0,
            "*" => 1,
            _   => panic!("unknown op {op}"),
        };

        for row in 0..rows-1 {
            let val: usize = table[row][col].parse().unwrap();
            match op {
                "+" => acc += val,
                "*" => acc *= val,
                _   => panic!("unknown op {op}")
                // _ is unreachable here.
            }
        }

        total += acc;
    }

    println!("Answer 1: {total}");
}

fn solve_2(input: &str) {
    let transposed = transpose(input)
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");

    let re = Regex::new(r"\d+").unwrap();

    let mut total = 0;
    for problem in transposed.split("\n\n") {
        let is_add = problem.contains("+");
        let mut acc = if is_add {0} else {1};

        for vals in re.find_iter(problem).map(|m| m.as_str()) {
            let val: usize = vals.parse().unwrap();
            if is_add {
                acc += val;
            } else {
                acc *= val;
            }
        }

        total += acc;
    }

    println!("Answer 2: {total}");
}



fn main() {
    let input = fs::read_to_string("input/day06.txt")
        .expect("failed to read file!");

    solve_1(&input);

    solve_2(&input);
}