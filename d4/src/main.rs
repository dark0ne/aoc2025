use std::fs;

fn main() {
    let input = fs::read_to_string("in1.txt").unwrap();
    let result = step1(&input);
    println!("step1: {result}");

    let result = step2(&input);
    println!("step2: {result}");
}

fn step1(input: &str) -> usize {
    let strings: Vec<&str> = input.split('\n').collect();

    let width = strings.get(0).unwrap().len();
    let height = strings.iter().filter(|s| !s.is_empty()).count();

    let mut field: Vec<bool> = Vec::with_capacity(width * height);

    for s in strings.iter().filter(|s| !s.is_empty()) {
        println!("={}=", s);
    }

    for c in strings
        .iter()
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.chars())
    {
        let v = match c {
            '.' => false,
            '@' => true,
            _ => panic!("wrong input"),
        };
        field.push(v);
    }

    let lookup = |x: isize, y: isize| -> usize {
        if x < 0 {
            0
        } else if x >= width as isize {
            0
        } else if y < 0 {
            0
        } else if y >= height as isize {
            0
        } else {
            if *field.get((x + y * width as isize) as usize).unwrap() {
                1
            } else {
                0
            }
        }
    };

    let mut count = 0;

    for y in 0..height as isize {
        for x in 0..width as isize {
            if lookup(x, y) != 0 {
                let mut neighbors = 0;
                for r_y in -1..2 {
                    for r_x in -1..2 {
                        if r_y == 0 && r_x == 0 {
                            continue;
                        }
                        neighbors += lookup(x + r_x, y + r_y);
                    }
                }
                print!("{neighbors}");
                if neighbors < 4 {
                    count += 1
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }

    count
}

fn step2(input: &str) -> usize {
    let strings: Vec<&str> = input.split('\n').collect();

    let width = strings.get(0).unwrap().len();
    let height = strings.iter().filter(|s| !s.is_empty()).count();

    let mut field: Vec<bool> = Vec::with_capacity(width * height);

    for s in strings.iter().filter(|s| !s.is_empty()) {
        println!("={}=", s);
    }

    for c in strings
        .iter()
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.chars())
    {
        let v = match c {
            '.' => false,
            '@' => true,
            _ => panic!("wrong input"),
        };
        field.push(v);
    }

    let lookup = |f: &Vec<bool>, x: isize, y: isize| -> usize {
        if x < 0 {
            0
        } else if x >= width as isize {
            0
        } else if y < 0 {
            0
        } else if y >= height as isize {
            0
        } else {
            if *f.get((x + y * width as isize) as usize).unwrap() {
                1
            } else {
                0
            }
        }
    };

    let mut count_total = 0;

    loop {
        let mut count_cur = 0;

        for y in 0..height as isize {
            for x in 0..width as isize {
                if lookup(&field, x, y) != 0 {
                    let mut neighbors = 0;
                    for r_y in -1..2 {
                        for r_x in -1..2 {
                            if r_y == 0 && r_x == 0 {
                                continue;
                            }
                            neighbors += lookup(&field, x + r_x, y + r_y);
                        }
                    }
                    print!("{neighbors}");
                    if neighbors < 4 {
                        count_cur += 1;
                        field[x as usize + y as usize * width] = false;
                    }
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        if count_cur == 0 {
            break;
        }
        count_total += count_cur;
    }

    count_total
}

#[test]
fn example_input_step1() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    assert_eq!(step1(input), 13);
}

#[test]
fn example_input_step2() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    assert_eq!(step2(input), 43);
}
