pub fn p1(input: String) -> u128 {
    // main difficulty of this problem is parsing the input
    let mut input_matrix: Vec<Vec<&str>> = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for v in line.split_ascii_whitespace() {
            row.push(v);
        }
        input_matrix.push(row);
    }

    let mut res = 0;
    for col in 0..input_matrix[0].len() {
        let operator = input_matrix[input_matrix.len() - 1][col];
        let mut total: u128 = if operator == "*" { 1 } else { 0 };
        for row in 0..input_matrix.len() - 1 {
            if operator == "*" {
                total *= input_matrix[row][col].parse::<u128>().unwrap();
            } else {
                total += input_matrix[row][col].parse::<u128>().unwrap();
            }
        }
        res += total;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            p1("123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
                .to_string()),
            4277556
        )
    }
}
