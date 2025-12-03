mod cpio;

#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(static_mut_refs)]

use std::io::{BufWriter, Write};
use std::str::FromStr;
use crate::cpio::*;

struct Input(char, usize);

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let dir = chars.next().ok_or("Missing direction")?;
        let num_str: String = chars.collect();
        let num = num_str.trim().parse::<usize>().map_err(|_| "Invalid number")?;
        Ok(Input(dir, num))
    }
}

sol_eof!(
    fn solution(nums: Input) -> String {
        let mut buf = BufWriter::new(Vec::new());

        static mut AT_ZERO: usize = 0;
        static mut CUR_POS: usize = 50;

        let at_zero = unsafe { &mut *&raw mut AT_ZERO };
        let cur_pos = unsafe { &mut *&raw mut CUR_POS };

        let Input(dir, num) = nums;
        write!(buf, "posb = {} ", *cur_pos).unwrap();
        let n = num % 100;
        match dir {
            'L' => { 
                *cur_pos = (*cur_pos + 100 - n) % 100; 
                write!(buf, "Left").unwrap()
            }
            'R' => { 
                *cur_pos = (*cur_pos + n) % 100; 
                write!(buf, "Right").unwrap() 
            }
            _ => panic!("Invalid direction"),
        }
        if *cur_pos == 0 {
            *at_zero += 1;
        }
        write!(buf, " {}: pos = {}, at_zero = {}", num, *cur_pos, *at_zero).unwrap();

        let bytes = buf.into_inner().unwrap();
        String::from_utf8(bytes).unwrap()
    }
);