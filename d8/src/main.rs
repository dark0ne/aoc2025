use std::{
    collections::{HashMap, HashSet},
    fs,
};
fn main() {
    let input = fs::read_to_string("in1.txt").unwrap();
    let result = step1(&input, 1000);
    println!("step1: {result}");

    let result = step2(&input);
    println!("step2: {result}");
}

#[derive(Clone, Copy, Hash)]
struct P {
    id: usize,
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
    let mut next_id = 0;

    let mut points = Vec::new();
    for l in lines.iter() {
        let nums: Vec<usize> = l.split(',').map(|n| n.parse().unwrap()).collect();
        let p = P {
            id: next_id,
            x: nums[0],
            y: nums[1],
            z: nums[2],
        };
        next_id += 1;

        points.push(p);
    }

    let mut distances = Vec::new();
    for (a, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(a + 1) {
            let dist = distance_sqr(p1, p2);
            distances.push((p1.id, p2.id, dist));
        }
    }

    distances.sort_by(|d1, d2| f32::total_cmp(&d1.2, &d2.2));

    let mut all_circuits: HashMap<usize, CircuitList> = HashMap::new();
    let mut point_to_circuit_size: HashMap<usize, usize> = HashMap::new();

    let mut single_list = CircuitList::new();
    for p in points.iter() {
        let mut single_circuit = Circuit::new();
        single_circuit.add_point(p.id);
        single_list.list.push(single_circuit);

        point_to_circuit_size.insert(p.id, 1);
    }

    all_circuits.insert(1, single_list);

    'skip: for dist in distances.iter().take(limit) {
        let p1_circ_size = point_to_circuit_size[&dist.0];
        let p2_circ_size = point_to_circuit_size[&dist.1];

        // Part of the same circuit?
        if p1_circ_size == p2_circ_size {
            for circuit in all_circuits[&p1_circ_size].list.iter() {
                if circuit.is_point_in(dist.0) && circuit.is_point_in(dist.1) {
                    continue 'skip;
                }
            }
        }

        let mut c1 = all_circuits
            .get_mut(&p1_circ_size)
            .unwrap()
            .remove_circuit_with_point(dist.0);

        let c2 = all_circuits
            .get_mut(&p2_circ_size)
            .unwrap()
            .remove_circuit_with_point(dist.1);

        all_circuits.retain(|_, list| !list.list.is_empty());

        c2.points.iter().for_each(|p| {
            c1.points.insert(*p);
        });
        let new_size = p1_circ_size + p2_circ_size;

        c1.points
            .iter()
            .for_each(|p| *point_to_circuit_size.get_mut(p).unwrap() = new_size);

        let entry = all_circuits.entry(new_size).or_insert(CircuitList::new());
        entry.list.push(c1);

        // dbg!(&all_circuits);
    }

    let mut keys = all_circuits
        .iter()
        .filter(|(_, c)| !c.list.is_empty())
        .map(|(k, _)| *k)
        .collect::<Vec<usize>>();
    keys.sort();
    keys.reverse();
    dbg!(&keys);
    keys.iter().take(3).product()
}

#[derive(Debug)]
struct Circuit {
    points: HashSet<usize>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            points: HashSet::new(),
        }
    }
    fn is_point_in(&self, p: usize) -> bool {
        self.points.contains(&p)
    }
    fn add_point(&mut self, p: usize) {
        self.points.insert(p);
    }
}

#[derive(Debug)]
struct CircuitList {
    list: Vec<Circuit>,
}

impl CircuitList {
    fn new() -> Self {
        Self { list: Vec::new() }
    }
    fn remove_circuit_with_point(&mut self, p: usize) -> Circuit {
        if let Some((idx, _)) = self.list.iter().enumerate().find(|(_, c)| c.is_point_in(p)) {
            return self.list.remove(idx);
        }
        panic!("Not found {p}");
    }
}

fn step2(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();
    let mut next_id = 0;

    let mut points = Vec::new();
    for l in lines.iter() {
        let nums: Vec<usize> = l.split(',').map(|n| n.parse().unwrap()).collect();
        let p = P {
            id: next_id,
            x: nums[0],
            y: nums[1],
            z: nums[2],
        };
        next_id += 1;

        points.push(p);
    }

    let mut distances = Vec::new();
    for (a, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(a + 1) {
            let dist = distance_sqr(p1, p2);
            distances.push((p1.id, p2.id, dist));
        }
    }

    distances.sort_by(|d1, d2| f32::total_cmp(&d1.2, &d2.2));

    let mut all_circuits: HashMap<usize, CircuitList> = HashMap::new();
    let mut point_to_circuit_size: HashMap<usize, usize> = HashMap::new();

    let mut single_list = CircuitList::new();
    for p in points.iter() {
        let mut single_circuit = Circuit::new();
        single_circuit.add_point(p.id);
        single_list.list.push(single_circuit);

        point_to_circuit_size.insert(p.id, 1);
    }

    all_circuits.insert(1, single_list);

    'skip: for dist in distances.iter() {
        let p1_circ_size = point_to_circuit_size[&dist.0];
        let p2_circ_size = point_to_circuit_size[&dist.1];

        // Part of the same circuit?
        if p1_circ_size == p2_circ_size {
            for circuit in all_circuits[&p1_circ_size].list.iter() {
                if circuit.is_point_in(dist.0) && circuit.is_point_in(dist.1) {
                    continue 'skip;
                }
            }
        }

        let mut c1 = all_circuits
            .get_mut(&p1_circ_size)
            .unwrap()
            .remove_circuit_with_point(dist.0);

        let c2 = all_circuits
            .get_mut(&p2_circ_size)
            .unwrap()
            .remove_circuit_with_point(dist.1);

        all_circuits.retain(|_, list| !list.list.is_empty());

        c2.points.iter().for_each(|p| {
            c1.points.insert(*p);
        });
        let new_size = p1_circ_size + p2_circ_size;

        c1.points
            .iter()
            .for_each(|p| *point_to_circuit_size.get_mut(p).unwrap() = new_size);

        let entry = all_circuits.entry(new_size).or_insert(CircuitList::new());
        entry.list.push(c1);

        if all_circuits
            .iter()
            .map(|(_, list)| list.list.len())
            .sum::<usize>()
            == 1
        {
            // we are done
            return points[dist.0].x * points[dist.1].x;
        }

        // dbg!(&all_circuits);
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
