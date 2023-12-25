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

/// Problem 4 from https://450dsa.com/bit_manipulation
pub fn is_power_of_two(n: u64) -> bool {
    count_set_bits(n) == 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_set_bits() {
        let n = 0b1011;
        assert_eq!(3, super::count_set_bits(n));

        let n = 0b0;
        assert_eq!(0, super::count_set_bits(n));
    }

    #[test]
    fn is_power_of_two() {
        let n = 0b1000;
        assert!(super::is_power_of_two(n));

        let n = 0b1001;
        assert!(!super::is_power_of_two(n));
    }

    #[test]
    fn bit_diff() {
        let a = 0b1001001;
        let b = 0b1111000;

        assert_eq!(super::bit_diff(a, b), 3);

        let a = 0b1001001;
        let b = 0b1001001;

        assert_eq!(super::bit_diff(a, b), 0);
    }
}
