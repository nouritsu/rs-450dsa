// Problem 3 from https://450dsa.com/search_sort
pub fn count_squares(until: u64) -> usize {
    let mut count = 0;

    for i in 1..=((until as f64).sqrt() as u64) {
        if i.pow(2) < until {
            count += 1;
        } else {
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_squares() {
        let n = 100;
        assert_eq!(9, super::count_squares(n));

        let n = 9;
        assert_eq!(2, super::count_squares(n));

        let n = 3;
        assert_eq!(1, super::count_squares(n));

        let n = 1;
        assert_eq!(0, super::count_squares(n));

        let n = 0;
        assert_eq!(0, super::count_squares(n));
    }
}
