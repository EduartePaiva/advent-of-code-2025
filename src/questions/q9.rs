pub fn p1(input: String) -> u128 {
    let mut max_size = 0;

    let mut values: Vec<(u128, u128)> = vec![];

    for l in input.lines() {
        let mut nums = l.split(',');
        let x = nums.next().unwrap().parse::<u128>().unwrap();
        let y = nums.next().unwrap().parse::<u128>().unwrap();
        values.push((x, y));
    }

    for i in 0..values.len() {
        for j in i + 1..values.len() {
            let (x1, y1) = values[i];
            let (x2, y2) = values[j];

            max_size = max_size.max((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1));
        }
    }

    max_size
}
pub fn p2(input: String) -> u128 {
    // we will build the matrix and paint the closed cycle.
    // for checking we can just check the boundary, that will optimize a bit
    // if the current rectangle is less sized than the current we have we can just skip it.
    // we can 'shrink' the rectangle, but Idk how to do that

    let mut max_size = 0;

    let mut values: Vec<(u128, u128)> = vec![];

    for l in input.lines() {
        let mut nums = l.split(',');
        let x = nums.next().unwrap().parse::<u128>().unwrap();
        let y = nums.next().unwrap().parse::<u128>().unwrap();
        values.push((x, y));
    }

    for i in 0..values.len() {
        for j in i + 1..values.len() {
            let (x1, y1) = values[i];
            let (x2, y2) = values[j];

            max_size = max_size.max((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1));
        }
    }

    max_size
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            p1("7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
            .to_string()),
            50
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            p2("7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
            .to_string()),
            24
        );
    }
}
