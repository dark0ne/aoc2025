use std::fs;

fn main() {
    let input = fs::read_to_string("in1.txt").unwrap();
    let result = step1(&input);
    println!("step1: {result}");

    let result = step2(&input);
    println!("step2: {result}");
}

fn step1(input: &str) -> usize {
    let banks: Vec<&str> = input.split('\n').collect();

    banks
        .iter()
        .filter(|b| !b.is_empty())
        .map(|b| {
            println!("{b}");
            let len = b.len();
            let (index, tens) = b
                .chars()
                .take(len - 1)
                .map(|c| c.to_digit(10).unwrap() as usize)
                .enumerate()
                .fold((0, 0), |(mut max_idx, mut max), (idx, v)| {
                    if v > max {
                        max_idx = idx;
                        max = v;
                    }
                    (max_idx, max)
                });
            println!("@{} {}", index, tens);
            let single = b
                .chars()
                .skip(index + 1)
                .map(|c| c.to_digit(10).unwrap() as usize)
                .max()
                .unwrap();
            println!("{}", single);
            tens * 10 + single
        })
        .sum()
}

fn step2(input: &str) -> usize {
    let banks: Vec<&str> = input.split('\n').collect();

    banks
        .iter()
        .filter(|b| !b.is_empty())
        .map(|b| {
            println!("{b}");

            let mut slice = *b;

            const DIGITS: usize = 12;
            let mut num = 0;
            for digit in 0..DIGITS {
                let left = DIGITS - digit - 1;
                let (index, max) = slice
                    .chars()
                    .take(slice.len() - left)
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .enumerate()
                    .fold((0, 0), |(mut max_idx, mut max), (idx, v)| {
                        if v > max {
                            max_idx = idx;
                            max = v;
                        }
                        (max_idx, max)
                    });

                slice = slice.split_at(index + 1).1;
                num = num * 10 + max;
            }
            println!("num {}", num);
            num
        })
        .sum()
}

#[test]
fn example_input_step1() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";

    assert_eq!(step1(input), 357);
}

#[test]
fn example_input_step2() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";

    assert_eq!(step2(input), 3121910778619);
}
