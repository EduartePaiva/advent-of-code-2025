pub fn p1(input: String) -> u64 {
    let mut sum = 0;
    for bank in input.lines() {
        let nums = bank.as_bytes();
        let bank_len = nums.len();
        let mut max_at_right: Vec<u8> = vec![0; bank_len];
        max_at_right[bank_len - 1] = nums[bank_len - 1];

        for i in (0..bank_len - 1).rev() {
            max_at_right[i] = nums[i].max(max_at_right[i + 1]);
        }
        let mut max_n = 0;
        for i in 0..bank_len - 1 {
            let left = (nums[i] - b'0') as u64 * 10;
            let right = (max_at_right[i + 1] - b'0') as u64;
            let n = left + right;

            max_n = max_n.max(n);
        }

        sum += max_n;
    }
    sum
}

pub fn p2(input: String) -> u128 {
    let mut sum = 0;

    // let's do a n^2 solution, the biggest leftmost number before remaining
    for bank in input.lines() {
        let nums = bank.as_bytes();
        let bank_len = nums.len();
        let mut result: Vec<u8> = Vec::with_capacity(12);

        let mut left_boundary = 0_usize;

        for rem in (0..12).rev() {
            let right_boundary = bank_len - rem;
            let mut max_n = 0;
            for i in left_boundary..right_boundary {
                if nums[i] > max_n {
                    max_n = nums[i];
                    left_boundary = i + 1;
                }
            }
            result.push(max_n);
            if result.len() == 12 {
                break;
            }
        }
        sum += result
            .iter()
            .map(|c| *c as char)
            .collect::<String>()
            .parse::<u128>()
            .unwrap()
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            p1("987654321111111
811111111111119
234234234234278
818181911112111"
                .to_string()),
            357
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            p2("987654321111111
811111111111119
234234234234278
818181911112111"
                .to_string()),
            3121910778619
        )
    }
}
