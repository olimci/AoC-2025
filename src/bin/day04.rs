use std::fs;

fn solve_1(input: &str) -> usize {
    let mut w: isize = 0;
    let mut h: isize = 0;
    let mut v = Vec::new();

    let n = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for line in input.lines() {
        let bs = line.as_bytes();
        if w == 0 {
            w = bs.len() as isize;
        }

        v.extend(bs.iter().map(|u| if *u == b'@' {1} else {0}));
        h += 1;
    }

    let mut count = 0;

    for i in 0..h {
        for j in 0..w {
            let mut nc: usize = 0;
            let idx: usize = (i * w + j) as usize;

            for (oi, oj) in n {
                let ni = (i as isize) + oi;
                let nj = (j as isize) + oj;

                if ni < 0 || nj < 0 || ni >= h || nj >= w {
                    continue;
                }

                let n_idx= (ni * w + nj) as usize;

                nc += v[n_idx];
            }

            if nc < 4 && v[idx] == 1 {
                count += 1;
                print!("x");
            } else if v[idx] == 1 {
                print!("@");
            } else {
                print!(".");
            }
        }

        println!("");
    }

    count
}

fn solve_2(input: &str) -> usize {
    let mut w: isize = 0;
    let mut h: isize = 0;
    let mut v = Vec::new();

    let n = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for line in input.lines() {
        let bs = line.as_bytes();
        if w == 0 {
            w = bs.len() as isize;
        }

        v.extend(bs.iter().map(|u| if *u == b'@' {1} else {0}));
        h += 1;
    }

    let mut removed: usize = 0;

    loop {
        let mut vn = v.clone();
        let mut changed = 0;

        for i in 0..h {
            for j in 0..w {
                let mut nc: usize = 0;
                let idx: usize = (i * w + j) as usize;

                for (oi, oj) in n {
                    let ni = (i as isize) + oi;
                    let nj = (j as isize) + oj;

                    if ni < 0 || nj < 0 || ni >= h || nj >= w {
                        continue;
                    }

                    let n_idx= (ni * w + nj) as usize;

                    nc += v[n_idx];
                }

                if nc < 4 && v[idx] == 1 {
                    vn[idx] = 0;
                    changed += 1;
                    removed += 1;
                }
            }
        }

        v = vn;

        if changed == 0 {
            break;
        }
    }

    removed
}

fn main() {
    let input = fs::read_to_string("input/day04.txt")
        .expect("failed to read file!");

    let ans_1 = solve_1(&input);
    println!("Part 1 answer:\n{ans_1}");

    println!("\n---\n");

    let ans_2 = solve_2(&input);
    println!("Part 2 answer:\n{ans_2}");
}