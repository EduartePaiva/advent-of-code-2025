pub fn p1(input: String) -> u64 {
    // parse the input. the overlapping ranges have to fuse together
    // ranges are inclusive

    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ingredients: Vec<u64> = vec![];
    let mut fresh_ingredients = 0;

    // parsing input
    let mut is_ranges = true;
    for line in input.lines() {
        if line.len() == 0 {
            is_ranges = false;
            continue;
        }

        if is_ranges {
            let (start, end) = line.split_once('-').unwrap();
            let range = (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());
            ranges.push(range);
        } else {
            ingredients.push(line.parse::<u64>().unwrap());
        }
    }
    ranges.sort();
    ingredients.sort();

    let mut range_index = 0;
    let mut ingredients_index = 0;

    while range_index < ranges.len() && ingredients_index < ingredients.len() {
        let (s, e) = ranges[range_index];
        let ingredient = ingredients[ingredients_index];

        if ingredient <= e && ingredient >= s {
            fresh_ingredients += 1;
            ingredients_index += 1;
            continue;
        }

        if ingredient < s {
            ingredients_index += 1;
            continue;
        }

        if ingredient > e {
            range_index += 1;
        }
    }

    fresh_ingredients
}

pub fn p2(input: String) -> u64 {
    // parse the input. the overlapping ranges have to fuse together
    // ranges are inclusive

    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut fresh_ingredients = 0;

    // parsing input
    for line in input.lines() {
        if line.len() == 0 {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        let range = (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());
        ranges.push(range);
    }
    ranges.sort();

    // aggregate all ranges
    let mut aggregated_ranges: Vec<(u64, u64)> = vec![];
    aggregated_ranges.push(ranges[0]);

    for i in 1..ranges.len() {
        let (cur_s, cur_e) = ranges[i];
        let &(_, prev_e) = aggregated_ranges.last().unwrap();

        if prev_e >= cur_s {
            let lst_idx = aggregated_ranges.len() - 1;
            aggregated_ranges[lst_idx].1 = cur_e.max(prev_e);
            continue;
        }

        aggregated_ranges.push(ranges[i]);
    }

    for (s, e) in aggregated_ranges.into_iter() {
        fresh_ingredients += e - s + 1;
    }

    fresh_ingredients
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            p1("3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            .to_string()),
            3
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            p2("3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            .to_string()),
            14
        );
    }
}
