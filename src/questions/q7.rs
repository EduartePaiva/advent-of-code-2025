use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub fn p1(input: String) -> u32 {
    // convert input in matrix
    let mut matrix = vec![];
    for l in input.lines() {
        let mut row = vec![];
        for c in l.chars() {
            row.push(c);
        }
        matrix.push(row);
    }

    let mut splitters_hit: HashSet<(usize, usize)> = HashSet::new();
    let mut visit: HashSet<(usize, usize)> = HashSet::new();

    // bfs
    fn travel(
        r: usize,
        c: usize,
        matrix: &Vec<Vec<char>>,
        splitters_hit: &mut HashSet<(usize, usize)>,
        visit: &mut HashSet<(usize, usize)>,
    ) {
        if let Some(_) = visit.get(&(r, c)) {
            return;
        } else {
            visit.insert((r, c));
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        if r >= rows || c >= cols {
            return;
        }

        if matrix[r][c] == '^' {
            splitters_hit.insert((r, c));
            if c > 0 {
                travel(r, c - 1, matrix, splitters_hit, visit);
            }

            if c < cols - 1 {
                travel(r, c + 1, matrix, splitters_hit, visit);
            }

            return;
        }

        travel(r + 1, c, matrix, splitters_hit, visit);
    }

    let c = matrix[0].iter().position(|v| *v == 'S').unwrap();
    travel(0, c, &matrix, &mut splitters_hit, &mut visit);

    splitters_hit.len() as u32
}
pub fn p2(input: String) -> u128 {
    // convert input in matrix
    let mut matrix = vec![];
    for l in input.lines() {
        let mut row = vec![];
        for c in l.chars() {
            row.push(c);
        }
        matrix.push(row);
    }

    let mut cache: HashMap<(usize, usize), u128> = HashMap::new();

    // now it's backtrack, how many paths does this split to right or left have?
    fn travel(
        r: usize,
        c: usize,
        matrix: &Vec<Vec<char>>,
        cache: &mut HashMap<(usize, usize), u128>,
    ) -> u128 {
        if let Some(&v) = cache.get(&(r, c)) {
            return v;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        if r >= rows {
            return 1;
        }

        if matrix[r][c] == '^' {
            let mut res = 0;
            if c > 0 {
                res += travel(r, c - 1, matrix, cache);
            }

            if c < cols - 1 {
                res += travel(r, c + 1, matrix, cache);
            }
            cache.insert((r, c), res);
            return res;
        }

        return travel(r + 1, c, matrix, cache);
    }

    let c = matrix[0].iter().position(|v| *v == 'S').unwrap();
    travel(0, c, &matrix, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            p1(".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
                .to_string()),
            21
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            p2(".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
                .to_string()),
            40
        )
    }
}
