use itertools::Itertools;
use std::{cmp::Ordering, collections::VecDeque, fs, io::Write, iter, usize};
fn main() {
    let input = fs::read_to_string("in1.txt").unwrap();
    let result = step1(&input);
    println!("step1: {result}");

    let result = step2(&input);
    println!("step2: {result}");
}

fn to_switch(s: &&str) -> usize {
    let Some(s) = s.strip_prefix('(') else {
        panic!("Missing (");
    };
    let Some(s) = s.strip_suffix(')') else {
        panic!("Missing )");
    };
    let nums: Vec<usize> = s.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    let mut switch = 0usize;
    for n in nums {
        switch |= 1 << n;
    }

    switch
}

fn to_switch_vec(s: &&str) -> Vec<usize> {
    let Some(s) = s.strip_prefix('(') else {
        panic!("Missing (");
    };
    let Some(s) = s.strip_suffix(')') else {
        panic!("Missing )");
    };
    let nums: Vec<usize> = s.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    nums
}

fn to_joltage(s: &str) -> Vec<usize> {
    let Some(s) = s.strip_prefix('{') else {
        panic!("Missing {{");
    };
    let Some(s) = s.strip_suffix('}') else {
        panic!("Missing }}");
    };
    s.split(',').map(|s| s.parse::<usize>().unwrap()).collect()
}
fn apply_joltage(j: &mut Vec<usize>, mut button: usize, times: usize, limits: &Vec<usize>) -> bool {
    let mut bit = 0;

    while button != 0 {
        let mask = 1 << bit;
        if button & mask != 0 {
            j[bit] += times;
            if j[bit] > limits[bit] {
                return true;
            }
            button ^= mask;
        }
        bit += 1;
    }
    false
}
fn compare_joltage(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    a.iter().zip(b.iter()).all(|(a, b)| *a == *b)
}

fn to_lights(s: &str) -> (usize, usize) {
    let Some(s) = s.strip_prefix('[') else {
        panic!("Missing [");
    };
    let Some(s) = s.strip_suffix(']') else {
        panic!("Missing ]");
    };
    let mut light = 0usize;
    for (idx, c) in s.chars().enumerate() {
        if c == '#' {
            light |= 1 << idx;
        }
    }
    (light, s.len())
}

fn step1(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();

    let mut pushes_total = 0;

    'next: for line in lines.iter() {
        let blocks: Vec<&str> = line.split_whitespace().collect();
        let (lights_end, len) = to_lights(blocks[0]);

        let switches = blocks
            .iter()
            .skip(1)
            .take(blocks.len() - 2)
            .map(to_switch)
            .collect::<Vec<_>>();

        for n in 1..switches.len() {
            for combo in (0..switches.len()).combinations(n) {
                let mut lights = 0;
                for push in combo {
                    lights ^= switches[push];
                }
                if lights == lights_end {
                    pushes_total += n;
                    continue 'next;
                }
            }
        }
    }

    pushes_total
}

fn step2(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();

    let mut pushes_total = 0;

    for line in lines.iter() {
        println!("{line}");

        let blocks: Vec<&str> = line.split_whitespace().collect();

        let switches = blocks
            .iter()
            .skip(1)
            .take(blocks.len() - 2)
            .map(to_switch_vec)
            .collect::<Vec<_>>();
        let joltage_end = to_joltage(blocks.last().unwrap());

        for (j, eq) in joltage_end.iter().enumerate() {
            for (i, sw) in switches.iter().enumerate() {
                if let Some(_) = sw.iter().find(|e| **e == j) {
                    print!("1 ")
                } else {
                    print!("0 ")
                }
            }
            println!("= {}", *eq);
        }

        if switches.len() > joltage_end.len() {
            pushes_total += 1;
        }
    }

    pushes_total
}

fn pr(pushes: &Vec<usize>) {
    print!("[ ");
    for e in pushes {
        print!("{e}, ")
    }
    println!("]")
}

fn change_pushes(v: &mut Vec<usize>) {
    let last_index = v.len() - 1;
    let last_elem = v[last_index];
    v[last_index] = 0;

    let f = v[..last_index].iter().enumerate().rfind(|(_, e)| **e != 0);
    match f {
        None => {
            v[0] = last_elem + 1;
        }
        Some((idx, _)) => {
            v[idx] -= 1;
            v[idx + 1] = last_elem + 1;
        }
    }
}

#[test]
fn example_input_step1() {
    let input = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    assert_eq!(step1(input), 7);
}

#[test]
fn example_input_step2() {
    let input = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    assert_eq!(step2(input), 33);
}

// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
//        a.  b.    c.  d.    e.    f
//        e+f = 3
//.       b+f = 5
//        c+d+e = 4
//        a+b+d = 7
