pub fn p1(file: String) -> u32 {
    // the dial range from 0 to 99. all of them starts at 50 R would mean increase the number
    // L would mean decrease.
    // it's a closed loop 99 -> 0 ... 0 <- 99
    // number of times after rotating that is left at 0, or that got to 0
    let mut times_of_0 = 0;
    let mut val = 50;

    for line in file.lines() {
        let mut num = line[1..].parse::<i32>().unwrap();
        if line.chars().next().unwrap() == 'L' {
            num *= -1;
        }

        val = (val + num) % 100;
        if val < 0 {
            val += 100;
        }

        if val == 0 {
            times_of_0 += 1;
        }
    }

    times_of_0
}

pub fn p2(file: String) -> u32 {
    let mut count = 0;
    let mut current = 50;

    for line in file.lines() {
        let value = line[1..].parse::<i32>().unwrap();
        let mut delta = 1;
        if line.chars().next().unwrap() == 'L' {
            delta = -1;
        }

        for _ in 0..value {
            current += delta;
            current %= 100;
            if current == 0 {
                count += 1;
            }
        }
    }

    count
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            p1("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            .to_string()),
            3
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            p2("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
            .to_string()),
            6
        )
    }
}
