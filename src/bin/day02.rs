use std::fs;

fn check_1(id: &usize) -> bool {
    let s = id.to_string();
    let m = s.len() / 2;

    s[0..m] == s[m..]
}

fn check_2(id: &usize) -> bool {
    let s = id.to_string();
    let l = s.len();
    
    if l == 1 { return false; }

    'outer: for i in 1..l/2+1 {
        if l % i != 0 { continue }
        
        for j in 1..l/i {
            if s[0..i] != s[j*i..(j+1)*i] { continue 'outer }
        }

        return true
    }

    return false
}

fn id_range(range: &str) -> std::ops::Range<usize> {
    let (fs, ts) = range.split_once('-').unwrap();
    let f: usize = fs.parse().unwrap();
    let t: usize = ts.parse().unwrap();

    f..t+1
}

fn part_1(input: &str) -> usize {
    input.split(",")
        .map(id_range)
        .map(|r| r.filter(check_1).sum::<usize>())
        .sum()
}

fn part_2(input: &str) -> usize {
    input.split(",")
        .map(id_range)
        .map(|r| r.filter(check_2).sum::<usize>())
        .sum()
}

fn main() {
    let input = fs::read_to_string("input/day02.txt")
        .expect("failed to read file!");

    let part_1_answer = part_1(&input);
    println!("Part 1 answer:\n{part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer:\n{part_2_answer}");
}