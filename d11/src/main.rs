use array2d::Array2D;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("in1.txt").unwrap();
    let result = step1(&input);
    println!("step1: {result}");

    let result = step2(&input);
    println!("step2: {result}");
}

fn step1(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();

    let name_to_index = {
        let mut map = HashMap::new();

        map.insert("out", 0usize);
        let mut next_index = 1usize;

        for line in lines.iter() {
            let (left, _rest) = line.split_once(": ").expect("Invalid input");
            let last_elem = map.insert(left, next_index);
            next_index += 1;
            assert!(last_elem.is_none());
        }
        map
    };

    let dim = name_to_index.len();
    let mut indexes = Array2D::filled_with(false, dim, dim);

    for line in lines.iter() {
        let (left, rest) = line.split_once(": ").expect("Invalid input");
        for right in rest.split_whitespace() {
            let left_idx = *name_to_index.get(left).unwrap();
            let right_idx = *name_to_index.get(right).unwrap();
            *indexes.get_mut(left_idx, right_idx).unwrap() = true;
        }
    }

    let visits = count("you", "out", &indexes, &name_to_index);

    visits
}

fn step2(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();

    let name_to_index = {
        let mut map = HashMap::new();

        map.insert("out", 0usize);
        let mut next_index = 1usize;

        for line in lines.iter() {
            let (left, _rest) = line.split_once(": ").expect("Invalid input");
            let last_elem = map.insert(left, next_index);
            next_index += 1;
            assert!(last_elem.is_none());
        }
        map
    };

    let dim = name_to_index.len();
    let mut indexes = Array2D::filled_with(false, dim, dim);

    for line in lines.iter() {
        let (left, rest) = line.split_once(": ").expect("Invalid input");
        for right in rest.split_whitespace() {
            let left_idx = *name_to_index.get(left).unwrap();
            let right_idx = *name_to_index.get(right).unwrap();
            *indexes.get_mut(left_idx, right_idx).unwrap() = true;
        }
    }

    let svr_fft = count("svr", "fft", &indexes, &name_to_index);
    dbg!(svr_fft);
    let fft_dac = count("fft", "dac", &indexes, &name_to_index);
    dbg!(fft_dac);
    let dac_out = count("dac", "out", &indexes, &name_to_index);
    dbg!(dac_out);

    svr_fft * fft_dac * dac_out
}

fn count(
    from: &str,
    to: &str,
    indexes: &Array2D<bool>,
    name_to_index: &HashMap<&str, usize>,
) -> usize {
    let start_idx = *name_to_index.get(from).unwrap();
    let end_idx = *name_to_index.get(to).unwrap();

    let dim = name_to_index.len();

    let mut visits_cache = Vec::new();
    visits_cache.resize(dim, None);
    visits_cache[end_idx] = Some(1);

    let mut q = VecDeque::new();
    q.push_back((start_idx, false));

    let mut cur_path: Vec<(usize, bool)> = Vec::new();

    const PRINT: bool = false;
    while !q.is_empty() {
        if PRINT {
            print!("queue ");
            for e in q.iter() {
                print!("{}{} ", e.0, if e.1 { "'" } else { "" });
            }
            println!("");
        }

        let (left, last) = q.pop_front().unwrap();

        cur_path.push((left, last));

        if PRINT {
            print!("path ");
            for e in cur_path.iter() {
                print!("{}{} ", e.0, if e.1 { "'" } else { "" });
            }
            println!("");
            print!("cache ");
            for e in visits_cache.iter().enumerate() {
                if let Some(v) = e.1 {
                    print!("{}: {}, ", e.0, v);
                }
            }
            println!("\n");
        }

        let mut new_subnodes = false;
        let mut first = true;

        for (right, _) in indexes
            .row_iter(left)
            .unwrap()
            .enumerate()
            .filter(|(_, connected)| **connected)
        {
            let v = *visits_cache.get(right).unwrap();
            if let Some(v) = v {
                if v != 0 {
                    for (n, _) in cur_path.iter() {
                        *visits_cache[*n].get_or_insert_default() += v;
                    }
                }
            } else {
                q.push_front((right, first));
                new_subnodes = true;
                if first {
                    first = false;
                }
            }
        }
        if !new_subnodes {
            let _ = visits_cache[left].get_or_insert_default();
            let mut extra_pop = true;
            while extra_pop {
                extra_pop = cur_path.pop().unwrap().1;
            }
        }
    }
    visits_cache[start_idx].unwrap()
}

#[test]
fn example_input_step1() {
    let input = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    assert_eq!(step1(input), 5);
}

#[test]
fn example_input_step2() {
    let input = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    assert_eq!(step2(input), 2);
}

// 3257661018750 too low
// 553204221431080 right
