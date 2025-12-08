fn is_roll_valid(grid: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    let dirs = [
        // up
        [-1, 0],
        // down
        [1, 0],
        // left
        [0, -1],
        // right
        [0, 1],
        // up+left
        [-1, -1],
        // up+right
        [-1, 1],
        // down+left
        [1, -1],
        // down+right
        [1, 1],
    ];

    let r_size = grid.len() as i32;
    let c_size = grid[0].len() as i32;

    let mut good = 0;
    for dir in dirs {
        let new_r = r as i32 + dir[0];
        let new_c = c as i32 + dir[1];

        if new_r >= 0
            && new_c >= 0
            && new_r < r_size
            && new_c < c_size
            && grid[new_r as usize][new_c as usize] == '@'
        {
            good += 1;
        }

        if good > 3 {
            return false;
        }
    }

    return true;
}

pub fn p1(input: String) -> u64 {
    let mut total = 0;
    let mut grid: Vec<Vec<char>> = vec![];

    // parse the input into the grid
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '@' && is_roll_valid(&grid, r, c) {
                total += 1;
            }
        }
    }

    total
}

pub fn p2(input: String) -> u64 {
    let mut total = 0;
    let mut grid: Vec<Vec<char>> = vec![];

    // parse the input into the grid
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    loop {
        let mut sub_total = 0;

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == '@' && is_roll_valid(&grid, r, c) {
                    sub_total += 1;
                    grid[r][c] = '.';
                }
            }
        }
        if sub_total > 0 {
            total += sub_total;
        } else {
            break;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            p1("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
                .to_string()),
            13
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            p2("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
                .to_string()),
            43
        );
    }
}
