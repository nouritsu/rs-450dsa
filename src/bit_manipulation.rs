/// Problem 0 from https://450dsa.com/bit_manipulation
pub fn count_set_bits(mut n: u64) -> usize {
    let mut count = 0usize;

    while n != 0 {
        count += (n & 0b1) as usize;
        n >>= 1;
    }

    count
}

/// Problem 2 from https://450dsa.com/bit_manipulation
pub fn bit_diff(a: u64, b: u64) -> usize {
    count_set_bits(a ^ b)
}

/// Problem 3 from https://450dsa.com/bit_manipulation
pub fn count_set_bits_till(n: u64) -> usize {
    if n == 0 {
        0
    } else {
        (1..=n).map(count_set_bits).sum()
    }
}

/// Problem 4 from https://450dsa.com/bit_manipulation
pub fn is_power_of_two(n: u64) -> bool {
    count_set_bits(n) == 1
}

/// Problem 5 from https://450dsa.com/bit_manipulation
pub fn bit_position(n: u64) -> Option<usize> {
    if n == 0 || !is_power_of_two(n) {
        return None;
    }

    for i in 0.. {
        if (n >> i) & 0b1 != 0 {
            return Some(i);
        }
    }

    None
}

/// Problem 7 from https://450dsa.com/bit_manipulation
pub fn div(mut a: i64, b: i64) -> Option<i64> {
    if b == 0 {
        return None;
    }

    let mut quotient = 0i64;
    while a >= b {
        if a - b >= 0 {
            a -= b;
            quotient += 1;
        }
    }
    Some(quotient)
}

/// Problem 8 from https://450dsa.com/bit_manipulation
pub fn square(mut n: isize) -> usize {
    if n < 0 {
        n *= -1;
    }

    let mut n2 = n as usize;

    for _ in 1..n {
        n2 += n as usize;
    }

    n2
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_set_bits() {
        let n = 0b1011;
        assert_eq!(super::count_set_bits(n), 3);

        let n = 0b0;
        assert_eq!(super::count_set_bits(n), 0);
    }

    #[test]
    fn is_power_of_two() {
        let n = 0b1000;
        assert!(super::is_power_of_two(n));

        let n = 0b1001;
        assert!(!super::is_power_of_two(n));
    }

    #[test]
    fn count_set_bits_till() {
        let n = 0;
        assert_eq!(0, super::count_set_bits_till(n));

        let n = 4;
        assert_eq!(5, super::count_set_bits_till(n));

        let n = 17;
        assert_eq!(35, super::count_set_bits_till(n));
    }

    #[test]
    fn bit_diff() {
        let a = 0b1001001;
        let b = 0b1111000;
        assert_eq!(3, super::bit_diff(a, b));

        let a = 0b1001001;
        let b = 0b1001001;
        assert_eq!(0, super::bit_diff(a, b));
    }

    #[test]
    fn square() {
        let n = 8;
        assert_eq!(64, super::square(n));

        let n = 0;
        assert_eq!(0, super::square(n));

        let n = -8;
        assert_eq!(64, super::square(n));
    }

    #[test]
    fn bit_position() {
        let n = 0;
        assert_eq!(None, super::bit_position(n));

        let n = 0b10;
        assert_eq!(Some(1), super::bit_position(n));

        let n = 0b1000;
        assert_eq!(Some(3), super::bit_position(n));

        let n = 0b101;
        assert_eq!(None, super::bit_position(n));
    }

    #[test]
    fn div() {
        let a = 1;
        let b = 0;
        assert_eq!(None, super::div(a, b));

        let a = 12;
        let b = 2;
        assert_eq!(Some(6), super::div(a, b));

        let a = 5;
        let b = 10;
        assert_eq!(Some(0), super::div(a, b))
    }
}
