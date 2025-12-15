use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn p1(input: String, connections: i128) -> u128 {
    let mut coords: Vec<(i128, i128, i128)> = vec![];
    for l in input.lines() {
        let mut nums = l.split(',');
        let x = nums.next().unwrap().parse::<i128>().unwrap();
        let y = nums.next().unwrap().parse::<i128>().unwrap();
        let z = nums.next().unwrap().parse::<i128>().unwrap();
        coords.push((x, y, z));
    }

    let mut min_heap: BinaryHeap<Reverse<(u128, (i128, i128, i128), (i128, i128, i128))>> =
        BinaryHeap::new();

    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let (x1, y1, z1) = coords[i];
            let (x2, y2, z2) = coords[j];

            let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);

            min_heap.push(Reverse((dist as u128, (x1, y1, z1), (x2, y2, z2))));
        }
    }

    let mut edges: HashMap<(i128, i128, i128), Vec<(i128, i128, i128)>> = HashMap::new();
    // any node in a island can be the entry point, I just have to add traveling up and down
    for _ in 0..connections {
        if let Some(Reverse((_, p1, p2))) = min_heap.pop() {
            let p1_edges = edges.entry(p1).or_insert(Vec::new());
            p1_edges.push(p2);
            let p2_edges = edges.entry(p2).or_insert(Vec::new());
            p2_edges.push(p1);
        } else {
            break;
        }
    }

    let mut visited: HashSet<(i128, i128, i128)> = HashSet::new();

    fn circuit_size(
        visited: &mut HashSet<(i128, i128, i128)>,
        edges: &HashMap<(i128, i128, i128), Vec<(i128, i128, i128)>>,
        cur: (i128, i128, i128),
    ) -> u128 {
        if !visited.insert(cur) {
            return 0;
        }

        let mut res = 1;
        if let Some(neighbors) = edges.get(&cur) {
            for nei in neighbors {
                res += circuit_size(visited, edges, *nei);
            }
        }

        res
    }

    let mut max1 = 1;
    let mut max2 = 1;
    let mut max3 = 1;

    for edge in coords {
        if visited.contains(&edge) {
            continue;
        }
        let size = circuit_size(&mut visited, &edges, edge);

        if size > max1 {
            max3 = max2;
            max2 = max1;
            max1 = size;
        } else if size > max2 {
            max3 = max2;
            max2 = size;
        } else if size > max3 {
            max3 = size;
        }
    }

    max1 * max2 * max3
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
