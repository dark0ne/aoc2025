use std::fs;

fn main() {
    let input = fs::read_to_string("in1.txt").unwrap();
    let result = step1(&input);
    println!("step1: {result}");

    let result = step2(&input);
    println!("step2: {result}");
}

fn step1(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();

    let num_lines = lines.len() - 1;

    let mut nums: Vec<Vec<usize>> = Vec::new();
    let cols = lines.get(0).unwrap().split_whitespace().count();
    nums.resize(cols, Vec::new());

    for l in lines.iter().take(num_lines) {
        for (n, v) in l.split_whitespace().zip(&mut nums) {
            v.push(n.parse().unwrap());
        }
    }

    lines
        .get(num_lines)
        .unwrap()
        .split_whitespace()
        .zip(&nums)
        .map(|(op, v)| match op {
            "+" => v.iter().fold(0, |sum, n| sum + n),
            "*" => v.iter().fold(1, |mult, n| mult * n),
            _ => panic!("Invalid operation"),
        })
        .map(|n| {
            println!("{n}");
            n
        })
        .sum()
}

fn step2(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();

    let num_lines = lines.len() - 1;

    let cols = lines.get(0).unwrap().split_whitespace().count();

    let mut nums: Vec<Vec<usize>> = Vec::new();
    nums.resize(cols, Vec::new());

    let mut op_positions: Vec<usize> = lines
        .get(num_lines)
        .unwrap()
        .match_indices(|c| c == '*' || c == '+')
        .map(|(p, _)| p)
        .collect();

    let mut op_line = *lines.get(num_lines).unwrap();
    let mut op_iter = op_positions.iter().enumerate().peekable();

    println!("{:?}", op_positions);

    let def = (op_positions.len(), op_line.len() + 1);
    while let Some((idx, &op_pos)) = op_iter.next() {
        let next_op_pos: usize = *op_iter.peek().unwrap_or(&(def.0, &def.1)).1;

        let diff = next_op_pos - op_pos - 1;

        nums[idx].resize(diff, 0);

        for pos in op_pos..next_op_pos - 1 {
            //println
            for l in lines.iter().take(num_lines) {
                let c = l.get(pos..pos + 1).unwrap().chars().next().unwrap();
                if c != ' ' {
                    let digit = c.to_digit(10).unwrap_or(0) as usize;
                    let elem = &mut nums[idx][pos - op_pos];
                    *elem = *elem * 10 + digit;
                }
            }
        }
    }

    print!("{:?}", nums);

    lines
        .get(num_lines)
        .unwrap()
        .split_whitespace()
        .zip(&nums)
        .map(|(op, v)| match op {
            "+" => v.iter().fold(0, |sum, n| sum + n),
            "*" => v.iter().fold(1, |mult, n| mult * n),
            _ => panic!("Invalid operation"),
        })
        .map(|n| {
            println!("{n}");
            n
        })
        .sum()
}

#[test]
fn example_input_step1() {
    let input = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    assert_eq!(step1(input), 4277556);
}

#[test]
fn example_input_step2() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    assert_eq!(step2(input), 3263827);
}
