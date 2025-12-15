use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub fn p1(input: String, connections: i32) -> i32 {
    let mut coords: Vec<(i32, i32, i32)> = vec![];
    for l in input.lines() {
        let mut nums = l.split(',');
        let x = nums.next().unwrap().parse::<i32>().unwrap();
        let y = nums.next().unwrap().parse::<i32>().unwrap();
        let z = nums.next().unwrap().parse::<i32>().unwrap();
        coords.push((x, y, z));
    }

    // now we connect the closest connection between two points 10 times.
    // then check how many islands is left.
    // don't repeat the same connection.
    // one node can connect to multiple nodes, but not the same pair.

    // getting all combinations ->

    let mut min_heap: BinaryHeap<Reverse<(u128, (i32, i32, i32), (i32, i32, i32))>> =
        BinaryHeap::new();

    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let (x1, y1, z1) = coords[i];
            let (x2, y2, z2) = coords[j];

            let sum = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
            let dist = ((sum as f64).sqrt() * 100.0) as u128;

            min_heap.push(Reverse((dist, (x1, y1, z1), (x2, y2, z2))));
        }
    }

    let mut edges: HashMap<(i32, i32, i32), Vec<(i32, i32, i32)>> = HashMap::new();
    // any node in a island can be the entry point, I just have to add traveling up and down
    for _ in 0..connections {
        if let Some(Reverse((_, p1, p2))) = min_heap.pop() {
            edges.get_mut(k)
        } else {
            break;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            p1(
                "162,817,812
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
425,690,689"
                    .to_string(),
                10
            ),
            40
        );
    }
}
