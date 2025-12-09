use std::fs;

fn make_grid(s: &str) -> (Vec<Vec<char>>, usize, usize) {
    let grid: Vec<Vec<char>> = s
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    assert!(!grid.is_empty(), "grid is empty!");

    let cols = grid[0].len();
    assert!(grid.iter().all(|row| row.len() == cols));
    let rows = grid.len();

    (grid, rows, cols)
}

fn make_zero_vec_2d(rows: usize, cols: usize) -> Vec<Vec<usize>> {
    (0..rows)
        .map(|_| {
            (0..cols)
                .map(|_| 0)
                .collect()
        })
        .collect()
}

fn solve(input: &str) {
    let (mut grid, rows, cols) = make_grid(input);

    let mut conn = make_zero_vec_2d(rows, cols);

    let mut split_count = 0;

    for i in 0..rows {
        for j in 0..cols {
            let n1 = if i >= 1 { grid[i-1][j] } else { '.' };
            let n2 = grid[i][j];

            let c1 = if i >= 1 { conn[i-1][j] } else { 0 };

            match (n1, n2) {
                ('S', '.') =>  {
                    grid[i][j] = '|';
                    conn[i][j] = 1;
                },
                ('|', '.') => {
                    grid[i][j] = '|';
                    conn[i][j] += c1;
                },
                ('|', '|') => {
                    conn[i][j] += c1;
                }
                ('|', '^') => {
                    grid[i][j-1] = '|';
                    grid[i][j+1] = '|';
                    conn[i][j-1] += c1;
                    conn[i][j+1] += c1;
                    split_count += 1;
                },
                _ => {},
            }
        }
    }

    let total_paths: usize = conn[rows-1].iter().sum();

    println!("Solution 1: {split_count}");
    println!("Solution 2: {total_paths}");
}


fn main() {
    let input = fs::read_to_string("input/day07.txt")
        .expect("failed to read file!");

    solve(&input);
}