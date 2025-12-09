use std::fs;
fn main() {
    let input = fs::read_to_string("in1.txt").unwrap();
    let result = step1(&input);
    println!("step1: {result}");

    let result = step2(&input);
    println!("step2: {result}");
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
struct P {
    x: usize,
    y: usize,
}

impl P {
    fn new(x: usize, y: usize) -> Self {
        P { x, y }
    }
}

fn area(p1: &P, p2: &P) -> usize {
    let dx = if p1.x > p2.x {
        p1.x - p2.x + 1
    } else {
        p2.x - p1.x + 1
    };
    let dy = if p1.y > p2.y {
        p1.y - p2.y + 1
    } else {
        p2.y - p1.y + 1
    };
    dx * dy
}

fn step1(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();

    let mut points = Vec::new();
    for l in lines.iter() {
        let nums: Vec<usize> = l.split(',').map(|n| n.parse().unwrap()).collect();
        points.push(P {
            x: nums[0],
            y: nums[1],
        });
    }

    let mut areas = Vec::new();
    for (p1_idx, p1) in points.iter().enumerate() {
        for (p2_idx, p2) in points.iter().enumerate().skip(p1_idx + 1) {
            let area = area(p1, p2);
            areas.push((p1_idx, p2_idx, area));
        }
    }
    *areas.iter().map(|(_, _, a)| a).max().unwrap()
}

fn min_max_pair(a: usize, b: usize) -> (usize, usize) {
    if a < b { (a, b) } else { (b, a) }
}

fn crosses_rect(r1: P, r2: P, a1: P, a2: P) -> bool {
    let rect_xs = min_max_pair(r1.x, r2.x);
    let rect_ys = min_max_pair(r1.y, r2.y);

    if a1.y == a2.y {
        // horizontal line
        let y = a1.y;
        // segment line lies on rect edge or is outside
        if rect_ys.0 >= y || y >= rect_ys.1 {
            return false;
        }
        let seg_xs = min_max_pair(a1.x, a2.x);
        // end of segement touches the rect edge, other end is outside
        if rect_xs.1 == seg_xs.0 || seg_xs.1 == rect_xs.0 {
            return false;
        }
        let range = seg_xs.0..=seg_xs.1;
        range.contains(&rect_xs.0) || range.contains(&rect_xs.1)
    } else {
        // a1.x == a2.x
        // vertical line
        let x = a1.x;
        // segment line lies on rect edge or is outside
        if rect_xs.0 >= x || x >= rect_xs.1 {
            return false;
        }
        let seg_ys = min_max_pair(a1.y, a2.y);
        // end of segement touches the rect edge, other end is outside
        if rect_ys.1 == seg_ys.0 || seg_ys.1 == rect_ys.0 {
            return false;
        }
        let range = seg_ys.0..=seg_ys.1;
        range.contains(&rect_ys.0) || range.contains(&rect_ys.1)
    }
}

fn step2(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();

    let mut points = Vec::new();
    for l in lines.iter() {
        let nums: Vec<usize> = l.split(',').map(|n| n.parse().unwrap()).collect();
        points.push(P::new(nums[0], nums[1]));
    }

    let mut segs = Vec::new();
    {
        let mut iter = points.iter().peekable();
        let first = points[0];
        while let Some(p1) = iter.next() {
            let p2 = **iter.peek().unwrap_or(&&first);
            segs.push((*p1, p2));
        }
    }

    let mut areas = Vec::new();
    for (r1_idx, r1) in points.iter().enumerate() {
        'next: for (_r2_idx, r2) in points.iter().enumerate().skip(r1_idx + 1) {
            if r1 == r2 {
                continue 'next;
            }

            let crosses = segs.iter().any(|seg| crosses_rect(*r1, *r2, seg.0, seg.1));
            if !crosses {
                let area = area(r1, &r2);
                areas.push((r1, r2, area));
            }
        }
    }
    *areas.iter().map(|(_, _, a)| a).max().unwrap()
}

#[test]
fn example_input_step1() {
    let input = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    assert_eq!(step1(input), 50);
}

#[test]
fn example_input_step2() {
    let input = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    assert_eq!(step2(input), 24);
}
/*
01234567890123
.............. 0
.......#XXX#.. 1
.......X...X.. 2
..#XXXX#...X.. 3
..X........X.. 4
..#XXXXXX#.X.. 5
.........X.X.. 6
.........#X#.. 7
.............. 8
*/
