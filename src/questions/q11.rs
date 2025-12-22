use std::collections::{HashMap, HashSet};

pub fn p1(input: String) -> u32 {
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in input.lines() {
        let items: Vec<_> = l.split_whitespace().collect();

        adj.entry(items[0].trim_end_matches(':'))
            .or_insert(Vec::new())
            .extend(items.iter().skip(1));
    }

    // we travel from you and finish at out.
    // what we want to know is how many paths can we make from you to out
    // we can't visit the same key two times, so if that happens it's an invalid path

    let mut visit: HashSet<&str> = HashSet::new();

    fn backtrack<'a>(
        adj: &HashMap<&str, Vec<&'a str>>,
        visit: &mut HashSet<&'a str>,
        cur: &'a str,
    ) -> u32 {
        if cur == "out" {
            return 1;
        }
        if visit.contains(cur) {
            return 0;
        }
        visit.insert(cur);

        let mut res = 0;
        if let Some(nei) = adj.get(cur) {
            for n in nei {
                res += backtrack(adj, visit, n);
            }
        }

        visit.remove(cur);
        res
    }

    backtrack(&adj, &mut visit, "you")
}
pub fn p2(input: String) -> u32 {
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in input.lines() {
        let items: Vec<_> = l.split_whitespace().collect();

        adj.entry(items[0].trim_end_matches(':'))
            .or_insert(Vec::new())
            .extend(items.iter().skip(1));
    }

    // we travel from svr and finish at out.
    // it's a valid path if we go thought dac and fft in any order.
    // what we want to know is how many paths can we make from you to out.
    // we can't visit the same key two times, so if that happens it's an invalid path.

    let mut visit: HashSet<&str> = HashSet::new();

    fn backtrack<'a>(
        adj: &HashMap<&str, Vec<&'a str>>,
        visit: &mut HashSet<&'a str>,
        cur: &'a str,
    ) -> u32 {
        if cur == "out" {
            if visit.contains("dac") && visit.contains("fft") {
                return 1;
            } else {
                return 0;
            }
        }
        if visit.contains(cur) {
            return 0;
        }

        visit.insert(cur);

        let mut res = 0;
        if let Some(nei) = adj.get(cur) {
            for n in nei {
                res += backtrack(adj, visit, n);
            }
        }

        visit.remove(cur);
        res
    }

    backtrack(&adj, &mut visit, "svr")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            p1("aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
                .to_string()),
            5
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            p2("svr: aaa bbb
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
hhh: out"
                .to_string()),
            2
        )
    }
}
