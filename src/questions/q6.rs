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

pub fn p2(input: String) -> u128 {
    // first rotate the matrix counter clockwise
    let mut input_matrix: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for v in line.chars() {
            row.push(v);
        }
        input_matrix.push(row);
    }

    let rows = input_matrix.len();
    let cols = input_matrix[0].len();

    let mut ccw_matrix = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            ccw_matrix[cols - 1 - j][i] = input_matrix[i][j];
        }
    }

    ccw_matrix.reverse();

    let rows = ccw_matrix.len();
    let cols = ccw_matrix[0].len();
    let mut total = 0;
    let mut res = 0;
    let mut cur_operator = '+';
    for r in 0..rows {
        // check if we have a new operator

        match ccw_matrix[r][cols - 1] {
            '+' => {
                cur_operator = '+';
                res += total;
                total = 0;
            }
            '*' => {
                cur_operator = '*';
                res += total;
                total = 1;
            }
            _ => (),
        }

        let num_str = &ccw_matrix[r][..cols - 1].iter().collect::<String>();
        let num_str = num_str.trim();
        if num_str.len() == 0 {
            continue;
        }

        if cur_operator == '*' {
            total *= num_str.parse::<u128>().unwrap();
        } else {
            total += num_str.parse::<u128>().unwrap();
        }
    }

    res += total;
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
    #[test]
    fn test2() {
        assert_eq!(
            p2("123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
                .to_string()),
            3263827
        )
    }
}
