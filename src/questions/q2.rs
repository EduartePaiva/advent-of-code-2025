pub fn p1(input: String) -> u64 {
    let mut sum = 0;

    for ranges in input.split(',') {
        let nums = ranges.split_once('-').unwrap();
        let num1 = nums.0.parse::<u64>().unwrap();
        let num2 = nums.1.parse::<u64>().unwrap();

        for n in num1..=num2 {
            let str_n = n.to_string();
            let byte_n = str_n.as_bytes();
            let mid = byte_n.len() / 2;
            if byte_n[..mid] == byte_n[mid..] {
                sum += n;
            }
        }
    }

    sum
}
pub fn p2(input: String) -> u64 {
    let mut sum = 0;

    for ranges in input.split(',') {
        let nums = ranges.split_once('-').unwrap();
        let num1 = nums.0.parse::<u64>().unwrap();
        let num2 = nums.1.parse::<u64>().unwrap();

        for n in num1..=num2 {
            let str_n = n.to_string();
            let byte_n = str_n.as_bytes();
            let mid = byte_n.len() / 2;

            for chunk in 1..=mid {
                let start_chunk = &byte_n[..chunk];

                let mut is_chunks_equal = true;
                for c in byte_n.chunks(chunk).skip(1) {
                    if c != start_chunk {
                        is_chunks_equal = false;
                        break;
                    }
                }
                if is_chunks_equal {
                    sum += n;
                    break;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            p1("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
                .to_string()),
            1227775554
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            p2("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
                .to_string()),
            4174379265
        )
    }
}
