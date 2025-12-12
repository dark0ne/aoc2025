use std::fs;

fn main() {
    let input = fs::read_to_string("in1.txt").unwrap();
    let result = step1(&input);
    println!("step1: {result}");
}

fn step1(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();

    let mut fits = 0;
    let mut not_fits = 0;
    for line in lines {
        let (coords, counts) = line.split_once(": ").unwrap();
        let (x, y) = coords.split_once('x').unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        let nums: Vec<usize> = counts
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect(&format!("not number {}", line)))
            .collect();

        let all_nums = nums.iter().sum();
        let place = (x / 3) * (y / 3);

        let parts: Vec<usize> = vec![7, 6, 5, 7, 7, 7];
        let fully_packed = nums
            .iter()
            .zip(parts.iter())
            .map(|(c, v)| c * v)
            .sum::<usize>();

        if place >= all_nums {
            fits += 1;
        } else if place < fully_packed {
            not_fits += 1;
        } else {
            println!("{all_nums} > {place}");
            println!("9*{all_nums} = {}, {x}*{y} = {}", 9 * all_nums, x * y);
            println!("fully_packed = {}, {x}*{y} = {}", 9 * all_nums, x * y);
            println!();
        }
    }

    dbg!(fits);
    dbg!(not_fits);

    fits
}
