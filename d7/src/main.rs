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
    let mut lines_it = lines.iter();
    let start_line = lines_it.next().unwrap();
    let cols = start_line.len();

    let mut splits = 0;
    let mut beams: Vec<bool> = Vec::new();
    beams.resize(cols, false);

    let start_col = start_line.find('S').unwrap();

    beams[start_col] = true;

    for line in lines_it {
        let mut beams_next = beams.clone();
        for (sep, _) in line.match_indices('^') {
            if beams_next[sep] {
                splits += 1;
                beams_next[sep] = false;
                if sep > 0 {
                    beams_next[sep - 1] = true;
                }
                if sep < cols - 1 {
                    beams_next[sep + 1] = true;
                }
            }
        }
        beams = beams_next;
    }

    splits
}
fn step2(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();
    let mut lines_it = lines.iter();
    let start_line = lines_it.next().unwrap();
    let cols = start_line.len();

    let mut ways = Vec::new();
    ways.resize(cols, 1usize);

    for line in lines.iter().skip(1).rev() {
        let mut copy = ways.clone();
        for (sep, _) in line.match_indices('^') {
            let v = ways[sep - 1] + ways[sep + 1];
            copy[sep] = v;
        }
        ways = copy;
    }

    *ways.iter().max().unwrap()
}

#[test]
fn example_input_step1() {
    let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    assert_eq!(step1(input), 21);
}

#[test]
fn example_input_step2() {
    let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    assert_eq!(step2(input), 40);
}
