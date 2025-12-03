use std::ops::Range;

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

fn is_even(n: usize) -> bool {
    n%2 == 0
}

fn filter(range: &Range<usize>) -> Option<Range<usize>> {
    let len_start = len(range.start);
    let len_end = len(range.end-1);
    match (is_even(len_start), is_even(len_end)) {
        (false, false) => None,
        (true, false) => Some(range.start..first_num_of_len(len_end)),
        (false, true) => Some(first_num_of_len(len_end)..range.end),
        (true, true) => Some(range.start..range.end),
    }

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

    for r in ranges2.iter().filter_map(filter) {
        println!("start {} (len {}), end {} (len {})", r.start, len(r.start), r.end-1, len(r.end-1));

        let l = len(r.start);
        let half = l/2;
        let factor = 10usize.pow(half as u32);

        let prefix = r.start / factor;
        let begin = prefix * factor + prefix;
        let increment = 1 * factor + 1;
        let mut new_start = begin;
        if r.start > begin {
            new_start += increment;
        }

        dbg!(begin);
        dbg!(increment);
        dbg!(new_start);
        for n in (new_start..r.end+1).step_by(increment) {
            println!("silly number {}", n);
            sum += n;
        }

    }

    println!("sum {}", sum);


}
