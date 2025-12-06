use std::{fs, ops::Range};

fn main() {
    let input = fs::read_to_string("in1.txt").unwrap();
    let result = step1(&input);
    println!("step1: {result}");

    let result = step2(&input);
    println!("step2: {result}");
}

fn step1(input: &str) -> usize {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    assert!(blocks.len() == 2);

    let ranges: Vec<Range<usize>> = blocks[0]
        .split('\n')
        .map(|l| {
            let mut it = l.split('-');
            let first = it.next().unwrap().parse::<usize>().unwrap();
            let last = it.next().unwrap().parse::<usize>().unwrap();
            first..last + 1
        })
        .collect();

    let count = blocks[1]
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<usize>().unwrap())
        .filter(|num| {
            for r in ranges.iter() {
                if r.contains(num) {
                    return true;
                }
            }
            return false;
        })
        .count();

    count
}

fn step2(input: &str) -> usize {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    assert!(blocks.len() == 2);

    let mut ranges: Vec<Range<usize>> = Vec::new();
    'input: for l in blocks[0].split('\n') {
        let mut cur_range = {
            let mut it = l.split('-');
            let first = it.next().unwrap().parse::<usize>().unwrap();
            let last = it.next().unwrap().parse::<usize>().unwrap();
            first..last + 1
        };
        println!("curr {}-{}", cur_range.start, cur_range.end - 1);
        for r in ranges.iter_mut() {
            if cur_range.contains(&r.start) && cur_range.contains(&(r.end - 1)) {
                *r = 1..1;
                continue;
            }
            println!("exis {}-{}", r.start, r.end - 1);
            if r.contains(&cur_range.start) {
                cur_range.start = r.end;
                println!("cut1 {}-{}", cur_range.start, cur_range.end - 1);
            }
            if r.contains(&(cur_range.end - 1)) {
                cur_range.end = r.start;
                println!("cut2 {}-{}", cur_range.start, cur_range.end - 1);
            }
        }
        if cur_range.start < cur_range.end {
            ranges.push(cur_range);
            ranges.sort_by_key(|r| r.start);
        }
        println!("");
    }

    for r in ranges.iter() {
        println!("{}-{}", r.start, r.end - 1);
    }

    ranges.iter().fold(0, |mut c, range| {
        c += range.end - range.start;
        c
    })
}

#[test]
fn example_input_step1() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    assert_eq!(step1(input), 3);
}

#[test]
fn example_input_step2() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    assert_eq!(step2(input), 14);
}
