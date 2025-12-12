use itertools::Itertools;
use std::{cmp::Ordering, collections::VecDeque, env::var, fs, io::Write, iter, usize};
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

const EPSILON: f64 = 1e-10f64;

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

        let rows = joltage_end.len();
        let cols = switches.len();
        let mut matrix: Vec<Vec<f64>> = Vec::new();
        matrix.resize(rows, Vec::new());

        for (row, &eq) in joltage_end.iter().enumerate() {
            for (i, sw) in switches.iter().enumerate() {
                let value = if let Some(_) = sw.iter().find(|e| **e == row) {
                    1.0f64
                } else {
                    0f64
                };
                matrix[row].push(value);
            }
            matrix[row].push(eq as f64);
        }

        let mut dependent = Vec::new();
        let mut independent = Vec::new();
        gauss(&mut matrix, &mut dependent, &mut independent);

        let mut variables = Vec::new();
        variables.resize(independent.len(), 0);

        let mut limits = Vec::new();
        for &v in independent.iter() {
            let limit = switches[v].iter().map(|&j| joltage_end[j]).max().unwrap();
            limits.push(limit);
        }

        let mut min = usize::MAX;
        if let Some(count) = test_variables(&matrix, &variables, &dependent, &independent) {
            min = min.min(count);
        }

        loop {
            if !change_variables(&mut variables, &limits) {
                break;
            }
            if variables.iter().sum::<usize>() >= min {
                continue;
            }
            if let Some(count) = test_variables(&matrix, &variables, &dependent, &independent) {
                min = min.min(count);
            }
        }
        println!("{min}");

        pushes_total += min;
    }

    pushes_total
}

const PRINT: bool = false;

fn gauss(m: &mut Vec<Vec<f64>>, dependent: &mut Vec<usize>, independent: &mut Vec<usize>) {
    let rows = m.len();
    let cols = m[0].len() - 1;
    let mut cur_row = 0;
    let mut cur_col = 0;
    while cur_row < rows && cur_col < cols {
        if PRINT {
            println!("row={cur_row}, col={cur_col}");
            print_matrix(m);
            println!();
        }

        let swap_row = m
            .iter()
            .enumerate()
            .skip(cur_row)
            .find(|(r, row)| row[cur_col].abs() > EPSILON)
            .map(|(r, _)| r);
        let Some(swap_row) = swap_row else {
            independent.push(cur_col);
            cur_col += 1;
            continue;
        };

        m.swap(cur_row, swap_row);

        // normalize
        let factor = m[cur_row][cur_col];
        for v in m[cur_row].iter_mut() {
            *v /= factor;
        }

        let calc = m[cur_row].clone();
        for (r, row) in m.iter_mut().enumerate() {
            if r != cur_row {
                let factor = row[cur_col];
                if factor.abs() > EPSILON {
                    for (&a, b) in calc.iter().skip(cur_col).zip(row.iter_mut().skip(cur_col)) {
                        *b -= factor * a;
                    }
                }
            }
        }
        dependent.push(cur_col);

        cur_row += 1;
        cur_col += 1;
    }

    println!("row={cur_row}, col={cur_col}");
    print_matrix(m);
    println!();

    independent.extend(cur_col..cols);
    println!("indep {:?}", independent);
}

fn test_variables(
    m: &Vec<Vec<f64>>,
    variables: &Vec<usize>,
    dependent: &Vec<usize>,
    independent: &Vec<usize>,
) -> Option<usize> {
    let mut count = variables.iter().sum();

    let cols = m[0].len() - 1;
    for r in 0..dependent.len() {
        let v = independent
            .iter()
            .enumerate()
            .fold(m[r][cols], |res, (value_idx, &var_idx)| {
                res - m[r][var_idx] * variables[value_idx] as f64
            });

        // Negative number is invalid
        if v < -EPSILON {
            return None;
        }

        // Non whole number is invalid
        let rounded_v = v.round();
        if (v - rounded_v).abs() > EPSILON {
            return None;
        }

        count += rounded_v as usize;
    }

    Some(count)
}

fn print_matrix(m: &Vec<Vec<f64>>) {
    for row in m.iter() {
        for &v in row.iter().take(row.len() - 1) {
            let v = if v.abs() < EPSILON { 0f64 } else { v };
            print!("{:>4} ", v);
        }
        println!("= {:>5}", row.last().unwrap());
    }
}

fn change_variables(v: &mut Vec<usize>, limits: &Vec<usize>) -> bool {
    let mut overflow = true;
    let mut index = 0;
    while overflow && index < v.len() {
        v[index] += 1;
        overflow = v[index] > limits[index];
        if overflow {
            v[index] = 0;
        }
        index += 1;
    }
    !overflow
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
