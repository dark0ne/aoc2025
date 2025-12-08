use disjoint::DisjointSet;
use std::{collections::HashSet, fs};
fn main() {
    let input = fs::read_to_string("in1.txt").unwrap();
    let result = step1(&input, 1000);
    println!("step1: {result}");

    let result = step2(&input);
    println!("step2: {result}");
}

#[derive(Clone, Copy, Hash)]
struct P {
    x: usize,
    y: usize,
    z: usize,
}

fn distance_sqr(p1: &P, p2: &P) -> f32 {
    let dx = p1.x as f32 - p2.x as f32;
    let dy = p1.y as f32 - p2.y as f32;
    let dz = p1.z as f32 - p2.z as f32;
    dx * dx + dy * dy + dz * dz
}

fn step1(input: &str, limit: usize) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();

    let mut points = Vec::new();
    for l in lines.iter() {
        let nums: Vec<usize> = l.split(',').map(|n| n.parse().unwrap()).collect();
        points.push(P {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        });
    }

    let mut distances = Vec::new();
    for (p1_idx, p1) in points.iter().enumerate() {
        for (p2_idx, p2) in points.iter().enumerate().skip(p1_idx + 1) {
            let dist = distance_sqr(p1, p2);
            distances.push((p1_idx, p2_idx, dist));
        }
    }

    distances.sort_by(|d1, d2| f32::total_cmp(&d1.2, &d2.2));

    let mut all_circuits = DisjointSet::with_len(points.len());

    for dist in distances.iter().take(limit) {
        let _ = all_circuits.join(dist.0, dist.1);
    }

    let sizes: HashSet<usize> = all_circuits.sets().into_iter().map(|v| v.len()).collect();

    let mut keys: Vec<usize> = sizes.into_iter().collect();
    keys.sort();
    keys.reverse();
    keys.iter().take(3).product()
}

fn step2(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();
    let mut points = Vec::new();
    for l in lines.iter() {
        let nums: Vec<usize> = l.split(',').map(|n| n.parse().unwrap()).collect();
        points.push(P {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        });
    }

    let mut distances = Vec::new();
    for (p1_idx, p1) in points.iter().enumerate() {
        for (p2_idx, p2) in points.iter().enumerate().skip(p1_idx + 1) {
            let dist = distance_sqr(p1, p2);
            distances.push((p1_idx, p2_idx, dist));
        }
    }

    distances.sort_by(|d1, d2| f32::total_cmp(&d1.2, &d2.2));

    let mut all_circuits = DisjointSet::with_len(points.len());

    for dist in distances.iter() {
        if all_circuits.join(dist.0, dist.1) {
            if all_circuits.sets().len() == 1 {
                return points[dist.0].x * points[dist.1].x;
            }
        }
    }

    panic!("Should not happen");
}

#[test]
fn example_input_step1() {
    let input = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    assert_eq!(step1(input, 10), 40);
}

#[test]
fn example_input_step2() {
    let input = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    assert_eq!(step2(input), 25272);
}
