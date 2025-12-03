use std::fs;

fn argmax(bs: &[u8]) -> (usize, u8) {
    let mut mi = 0;
    let mut mb = 0;

    for (i, b) in bs.iter().enumerate() {
        if *b > mb {
            mb = *b;
            mi = i;
        }
    }

    (mi, mb)
}

fn solve(input: &str, n: usize) -> usize {
    let scl_0 = usize::pow(10, (n-1) as u32);
    let max_j = |s: &str| {
        let bs = s.as_bytes();
        let bl = bs.len();

        let mut acc: usize = 0;
        let mut scl: usize = scl_0;
        let mut off: usize = 0;

        for i in 0..n {
            let r = bl + i + 1 - n;
            let (i, b) = argmax(&bs[off..r]);

            off += i+1;
            acc += ((b - b'0') as usize)*scl;
            scl /= 10;
        }

        acc
    };

    input.lines().map(max_j).sum()
}

fn main() {
    let input = fs::read_to_string("input/day03.txt")
        .expect("failed to read file!");

    let part_1_answer = solve(&input, 2);
    println!("Part 1 answer:\n{part_1_answer}");

    let part_2_answer = solve(&input, 12);
    println!("Part 2 answer:\n{part_2_answer}");
}