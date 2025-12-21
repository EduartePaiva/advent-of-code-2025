use std::collections::HashMap;

pub fn p1(input: String) -> u32 {
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in input.lines() {
        let items: Vec<_> = l.split_whitespace().collect();

        let items_left = items[1..].to_vec();
        adj.entry(items[0].trim_end_matches(':'))
            .or_insert(Vec::new())
            .extend(items_left);
    }

    dbg!(adj);

    0
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
}
