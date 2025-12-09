use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Vector3 {
    x: isize,
    y: isize,
    z: isize,
}

fn parse_input(input: &str) -> Vec<Vector3> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(",").map(|part| part.parse().unwrap());
            Vector3 {
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
                z: parts.next().unwrap(),
            }
        })
        .collect()
}

fn make_dist_map(nodes: Vec<Vector3>) -> Vec<(usize, usize, isize)> {
    let mut dists = Vec::new();

    for (i, a) in nodes.iter().enumerate() {
        dists.extend(
            nodes
                .iter()
                .enumerate()
                .map(|(j, b)| {
                    if i >= j {
                        (i, j, 0)
                    } else {
                        (
                            i,
                            j,
                            (a.x - b.x) * (a.x - b.x)
                                + (a.y - b.y) * (a.y - b.y)
                                + (a.z - b.z) * (a.z - b.z),
                        )
                    }
                })
                .filter(|a| a.2 != 0),
        );
    }

    dists.sort_by(|a: &(usize, usize, isize), b: &(usize, usize, isize)| b.2.cmp(&a.2));

    dists
}

fn solve(input: &str) {
    let nodes = parse_input(input);
    let mut dists = make_dist_map(nodes.clone());
    let mut comps: HashMap<usize, usize> = HashMap::new();

    let mut next_comp = 0;

    let n = 1000;
    let mut count = 0;

    while let Some((i, j, _dist)) = dists.pop() {
        let ci = comps.get(&i).cloned();
        let cj = comps.get(&j).cloned();

        let merged = match (ci, cj) {
            (Some(ci), Some(cj)) if ci == cj => false,
            (Some(ci), Some(cj)) => {
                for v in comps.values_mut() {
                    if *v == ci {
                        *v = cj;
                    }
                }
                true
            }
            (Some(ci), None) => {
                comps.insert(j, ci);
                true
            }
            (None, Some(cj)) => {
                comps.insert(i, cj);
                true
            }
            (None, None) => {
                comps.insert(i, next_comp);
                comps.insert(j, next_comp);
                next_comp += 1;
                true
            }
        };

        if count == n {
            let comp_sizes = comps.values().counts();
            let order = comp_sizes.values().sorted().rev().collect::<Vec<_>>();

            println!("[GREP TARGET] {:?}", order); //lol
        }
        count += 1;

        if merged {
            println!("Merged {:?} and {:?}", nodes[i], nodes[j]);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input/day08.txt").expect("Failed to read input file");

    solve(&input);
}
