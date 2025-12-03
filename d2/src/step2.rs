use std::{collections::HashSet, ops::Range};

macro_rules! r {
    ($start:literal-$end:literal) => {
        $start..$end + 1
    };
}

fn len(n: usize) -> usize {
    match n {
        0..10 => 1,
        10..100 => 2,
        100..1_000 => 3,
        1_000..10_000 => 4,
        10_000..100_000 => 5,
        100_000..1_000_000 => 6,
        1_000_000..10_000_000 => 7,
        10_000_000..100_000_000 => 8,
        100_000_000..1_000_000_000 => 9,
        1_000_000_000..10_000_000_000 => 10,
        _ => panic!("too large number"),
    }
}

fn first_num_of_len(len: usize) -> usize {
    10usize.pow(len as u32 - 1)
}

fn split(range: &Range<usize>) -> Vec<Range<usize>> {
    let len_start = len(range.start);
    let len_end = len(range.end - 1);

    if len_start == len_end {
        return vec![range.start..range.end];
    }

    let mut res = vec![range.start..first_num_of_len(len_start + 1)];

    for len in len_start + 1..len_end {
        res.push(first_num_of_len(len)..first_num_of_len(len + 1));
    }
    res.push(first_num_of_len(len_end)..range.end);

    res
}

fn to_num(part: usize, width: usize, reps: usize) -> usize {
    let factor = 10usize.pow(width as u32);
    let mut n = 0;
    for _ in 0..reps {
        n = n * factor + part
    }
    n
}

fn main() {
    let ranges: Vec<Range<usize>> = vec![
        r!(11 - 22),
        r!(95 - 115),
        r!(998 - 1012),
        r!(1188511880 - 1188511890),
        r!(222220 - 222224),
        r!(1698522 - 1698528),
        r!(446443 - 446449),
        r!(38593856 - 38593862),
        r!(565653 - 565659),
        r!(824824821 - 824824827),
        r!(2121212118 - 2121212124),
    ];

    let ranges2: Vec<Range<usize>> = vec![
        r!(1 - 18),
        r!(23 - 48),
        r!(58 - 72),
        r!(73 - 97),
        r!(149 - 216),
        r!(294 - 400),
        r!(538 - 812),
        r!(1095 - 1358),
        r!(1491 - 1766),
        r!(1894 - 2622),
        r!(3308 - 4582),
        r!(5735 - 8423),
        r!(9123 - 12332),
        r!(53435 - 76187),
        r!(81867 - 97148),
        r!(196475 - 300384),
        r!(309711 - 443410),
        r!(523453 - 569572),
        r!(727833 - 843820),
        r!(852101 - 903928),
        r!(1007333 - 1150296),
        r!(2131524 - 2335082),
        r!(3511416 - 3689352),
        r!(4394592 - 4512674),
        r!(15609927 - 15646018),
        r!(58406664 - 58466933),
        r!(62064792 - 62301480),
        r!(67562556 - 67743658),
        r!(69552998 - 69828126),
        r!(648635477 - 648670391),
        r!(6767640219 - 6767697605),
        r!(698652479 - 698760276),
        r!(2929221721 - 2929361280),
        r!(7979723815 - 7979848548),
        r!(9574291560 - 9574498524),
    ];

    let mut sum = 0;

    for r in ranges2.iter().flat_map(|r| split(r)) {
        println!(
            "start {} (len {}), end {} (len {})",
            r.start,
            len(r.start),
            r.end - 1,
            len(r.end - 1)
        );

        let l = len(r.start);

        let mut used = HashSet::new();

        for reps in len_to_valid_reps(l) {
            for n in SillyIter::new(r.start, r.end, reps) {
                if !used.contains(&n) {
                    used.insert(n);
                    println!("silly {}", n);
                    sum += n;
                }
            }
        }
    }

    println!("sum {}", sum);
}

fn len_to_valid_reps(len: usize) -> Vec<usize> {
    match len {
        1 => vec![],
        2 => vec![2],
        3 => vec![3],
        4 => vec![2, 4],
        5 => vec![5],
        6 => vec![2, 3, 6],
        7 => vec![7],
        8 => vec![2, 4, 8],
        9 => vec![3, 9],
        10 => vec![2, 5, 10],
        _ => panic!("Invalid length."),
    }
}

struct SillyIter {
    cur: usize,
    increment: usize,
    limit: usize,
}

impl SillyIter {
    fn new(start: usize, limit: usize, reps: usize) -> Self {
        let width = len(start) / reps;
        let increment = to_num(1, width, reps);
        let part = start / 10usize.pow((width * (reps - 1)) as u32);
        let mut first = to_num(part, width, reps);
        if start > first {
            first += increment;
        }
        Self {
            cur: first,
            increment,
            limit,
        }
    }
}

impl Iterator for SillyIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.limit {
            return None;
        }
        let next = self.cur;
        self.cur += self.increment;
        Some(next)
    }
}
