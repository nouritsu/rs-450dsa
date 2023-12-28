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
        assert_eq!(super::count_squares(100), 9);
        assert_eq!(super::count_squares(9), 2);
        assert_eq!(super::count_squares(3), 1);
        assert_eq!(super::count_squares(1), 0);
        assert_eq!(super::count_squares(0), 0);
    }
}
