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

        let opt = z3::Optimize::new();

        let vars: Vec<_> = (0..switches.len())
            .map(|i| z3::ast::Int::new_const(i as u32))
            .collect();

        for var in &vars {
            opt.assert(&var.ge(&z3::ast::Int::from_i64(0)));
        }

        for (i, &joltage) in joltage_end.iter().enumerate() {
            let joltage = joltage as u64;
            let mut button_mapping_to_joltage = Vec::new();
            for (j, button_mapping) in switches.iter().enumerate() {
                for &button in button_mapping.iter() {
                    if button == i {
                        button_mapping_to_joltage.push(vars[j].clone());
                    }
                }
            }

            let sum = button_mapping_to_joltage
                .into_iter()
                .reduce(|a, b| a + b)
                .unwrap();
            let target_joltage = z3::ast::Int::from_u64(joltage);
            opt.assert(&sum.eq(&target_joltage));
        }

        let button_presses: z3::ast::Int =
            vars.iter().map(|v| v.clone()).reduce(|a, b| a + b).unwrap();
        opt.minimize(&button_presses);

        if opt.check(&[]) == z3::SatResult::Sat {
            let pushes = opt
                .get_model()
                .unwrap()
                .eval(&button_presses, false)
                .unwrap()
                .as_u64()
                .unwrap() as usize;

            println!("{pushes}");
            pushes_total += pushes;
        }
    }

    pushes_total
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
