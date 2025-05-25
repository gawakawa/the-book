use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 2, 3];
    println!("v = {:?}", v);
    println!("mean of v = {:?}", mean(&v));
    println!("median of v = {:?}", median(&v));
    println!("mode of v = {:?}", mode(&v));
}

fn mean(ns: &[i32]) -> Option<f64> {
    if ns.is_empty() {
        return None;
    }

    let sum: i32 = ns.iter().sum();
    Some(sum as f64 / ns.len() as f64)
}

fn median(ns: &[i32]) -> Option<f64> {
    if ns.is_empty() {
        return None;
    }

    let mut sorted = ns.to_vec();
    sorted.sort();

    let len = ns.len();
    let mid = len / 2;
    if len % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) as f64 / 2.0)
    } else {
        Some(sorted[mid] as f64)
    }
}

fn mode(ns: &Vec<i32>) -> Vec<i32> {
    if ns.is_empty() {
        return vec![];
    }

    let mut counts = HashMap::new();
    for &n in ns {
        *counts.entry(n).or_insert(0) += 1;
    }

    let max_count = *counts.values().max().unwrap();

    counts
        .into_iter()
        .filter(|(_, count)| *count == max_count)
        .map(|(n, _)| n)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_empty() {
        let numbers = vec![];
        assert_eq!(mean(&numbers), None);
    }

    #[test]
    fn test_mean_single_value() {
        let numbers = vec![5];
        assert_eq!(mean(&numbers), Some(5.0));
    }

    #[test]
    fn test_mean_positive_values() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(mean(&numbers), Some(3.0));
    }

    #[test]
    fn test_mean_negative_values() {
        let numbers = vec![-10, -20, -30];
        assert_eq!(mean(&numbers), Some(-20.0));
    }

    #[test]
    fn test_median_empty() {
        let numbers = vec![];
        assert_eq!(median(&numbers), None);
    }

    #[test]
    fn test_median_single_value() {
        let numbers = vec![42];
        assert_eq!(median(&numbers), Some(42.0));
    }

    #[test]
    fn test_median_odd_count() {
        let numbers = vec![3, 1, 4, 5, 2];
        assert_eq!(median(&numbers), Some(3.0));
    }

    #[test]
    fn test_median_even_count() {
        let numbers = vec![3, 1, 4, 2];
        assert_eq!(median(&numbers), Some(2.5));
    }

    #[test]
    fn test_mode_empty() {
        let numbers = vec![];
        assert_eq!(mode(&numbers), Vec::<i32>::new());
    }

    #[test]
    fn test_mode_single_value() {
        let numbers = vec![7];
        assert_eq!(mode(&numbers), vec![7]);
    }

    #[test]
    fn test_mode_unique_values() {
        let numbers = vec![1, 2, 3];
        let result = mode(&numbers);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_mode_multiple_modes() {
        let numbers = vec![1, 1, 2, 2, 3];
        let result = mode(&numbers);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_mode_single_mode() {
        let numbers = vec![1, 2, 2, 3];
        assert_eq!(mode(&numbers), vec![2]);
    }
}
