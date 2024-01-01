use std::cmp::Ordering;

/// Problem 0 from https://450dsa.com/array
pub fn reverse_array<T>(xs: &mut [T]) {
    if xs.len() > 0 {
        let mut i = 0;
        let mut j = xs.len() - 1;

        while i < j {
            xs.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

/// Problem 1 from https://450dsa.com/array
pub fn min_and_max<T>(xs: &[T]) -> Option<(T, T)>
where
    T: Ord + Clone,
{
    if xs.len() == 0 {
        return None;
    }

    let mut min = &xs[0];
    let mut max = &xs[0];

    for i in 0..xs.len() {
        if let Ordering::Greater = min.cmp(&xs[i]) {
            min = &xs[i];
        }

        if let Ordering::Less = max.cmp(&xs[i]) {
            max = &xs[i];
        }
    }

    Some((min.clone(), max.clone()))
}

/// Problem 2 https://450dsa.com/array
pub fn kth_min_and_max<T>(xs: &[T], k: usize) -> Option<(T, T)>
where
    T: Ord + Clone,
{
    if xs.len() == 0 || xs.len() < k || k == 0 {
        return None;
    }

    let mut sorted_xs = Vec::from(xs);
    sorted_xs.sort();

    return Some((
        sorted_xs[k - 1].clone(),
        sorted_xs[sorted_xs.len() - k].clone(),
    ));
}

#[cfg(test)]
mod tests {

    #[test]
    fn reverse_array() {
        let mut xs: [i32; 0] = [];
        super::reverse_array(&mut xs);
        assert_eq!(xs, []);

        let mut xs = [1, 2, 3, 4, 5];
        super::reverse_array(&mut xs);
        assert_eq!(xs, [5, 4, 3, 2, 1])
    }

    #[test]
    fn min_and_max() {
        let xs = [1, 1, 2, 3, 5, 8, 13, 21];
        assert_eq!(Some((1, 21)), super::min_and_max(&xs));

        let xs: [i32; 0] = [];
        assert_eq!(None, super::min_and_max(&xs));
    }

    #[test]
    fn kth_min_and_max() {
        let xs = [1, 1, 2, 3, 5, 8, 13, 21];
        assert_eq!(Some((2, 8)), super::kth_min_and_max(&xs, 3));

        let xs = [1, 1, 2, 3, 5, 8, 13, 21];
        assert_eq!(super::min_and_max(&xs), super::kth_min_and_max(&xs, 1));

        let xs = [1, 1, 2, 3, 5, 8, 13, 21];
        assert_eq!(
            super::min_and_max(&xs).map(|(min, max)| (max, min)),
            super::kth_min_and_max(&xs, 8)
        );

        let xs: [i32; 0] = [];
        assert_eq!(None, super::kth_min_and_max(&xs, 1));

        let xs = [1, 1, 2, 3, 5, 8, 13, 21];
        assert_eq!(None, super::kth_min_and_max(&xs, 0));

        let xs = [1, 1, 2, 3, 5, 8, 13, 21];
        assert_eq!(None, super::kth_min_and_max(&xs, 9))
    }
}
