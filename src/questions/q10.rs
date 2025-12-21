use std::{collections::HashMap, u32};

#[derive(Debug)]
struct InputP1 {
    pattern: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}
#[derive(Debug)]
struct InputP2 {
    pattern: Vec<u32>,
    buttons: Vec<Vec<usize>>,
}

fn solve_p1(inp: InputP1) -> u32 {
    let mut hash: HashMap<Vec<bool>, u32> = HashMap::new();

    fn dfs(hash: &mut HashMap<Vec<bool>, u32>, pat: Vec<bool>, btns: &Vec<Vec<usize>>, depth: u32) {
        for b in btns {
            let mut new_p = pat.clone();
            for i in b {
                new_p[*i] = !new_p[*i];
            }

            if let Some(v) = hash.get(&new_p) {
                if *v <= depth + 1 {
                    continue;
                }
            }

            hash.insert(new_p.clone(), depth + 1);
            dfs(hash, new_p, btns, depth + 1);
        }
    }

    let start: Vec<bool> = (0..inp.pattern.len()).map(|_| false).collect();

    dfs(&mut hash, start, &inp.buttons, 0);

    *hash.get(&inp.pattern).unwrap()
}

fn solve_p2(inp: InputP2) -> u32 {
    let mut cache: HashMap<Vec<u32>, u32> = HashMap::new();

    dbg!(&inp.pattern);

    // ask how many steps for this pattern, or this other one return min
    fn backtack(cache: &mut HashMap<Vec<u32>, u32>, pat: Vec<u32>, btns: &Vec<Vec<usize>>) -> u32 {
        // if we emptied it means we have 0 steps left
        if pat.iter().sum::<u32>() == 0 {
            return 0;
        }

        if let Some(v) = cache.get(&pat) {
            return *v;
        }

        let mut min_depth = u32::MAX;
        for b in btns {
            let mut new_p = pat.clone();
            let mut is_valid = true;
            for i in b {
                if new_p[*i] == 0 {
                    is_valid = false;
                    break;
                }

                new_p[*i] -= 1;
            }

            if !is_valid {
                continue;
            }

            min_depth = min_depth.min(backtack(cache, new_p, btns));
        }

        if min_depth == u32::MAX {
            cache.insert(pat, u32::MAX);
            return u32::MAX;
        }
        cache.insert(pat, min_depth + 1);

        min_depth + 1
    }

    return backtack(&mut cache, inp.pattern, &inp.buttons);
}

pub fn p1(input: String) -> u32 {
    let mut parsed_inp: Vec<InputP1> = vec![];

    // parse input
    for l in input.lines() {
        let mut patterns = l.split_whitespace();

        // extract first pattern
        let pattern: Vec<bool> = patterns
            .next()
            .unwrap()
            .trim_matches(['[', ']'])
            .chars()
            .map(|c| c == '#')
            .collect();

        let buttons: Vec<Vec<usize>> = patterns
            .filter(|v| v.chars().next().unwrap() != '{')
            .map(|v| {
                v.trim_matches(['(', ')'])
                    .split(',')
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        parsed_inp.push(InputP1 { pattern, buttons });
    }

    parsed_inp.into_iter().map(solve_p1).sum()
}

pub fn p2(input: String) -> u32 {
    // same thing as the previous one, but now we break the dfs once any number increase goes past the max
    let mut parsed_inp: Vec<InputP2> = vec![];

    // parse input
    for l in input.lines() {
        let mut patterns: Vec<_> = l.split_whitespace().collect();
        patterns.remove(0);

        let pattern = patterns[patterns.len() - 1]
            .trim_matches(['{', '}'])
            .split(',')
            .map(|v| v.parse::<u32>().unwrap())
            .collect();

        let buttons: Vec<Vec<usize>> = patterns
            .iter()
            .filter(|v| v.chars().next().unwrap() != '{')
            .map(|v| {
                v.trim_matches(['(', ')'])
                    .split(',')
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        parsed_inp.push(InputP2 { pattern, buttons });
    }

    parsed_inp.into_iter().map(solve_p2).sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            p1("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
                .to_string()),
            7
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            p2("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
                .to_string()),
            33
        )
    }
}
