use quickcheck::{quickcheck, TestResult};
use crate::stat_funcs::{
    kth_stats_recur, mean, median_grouped, partition, pvariance, variance,
};
use ordered_float::*;

// round number up to $digits digits, convenient for some tests below
macro_rules! round {
    ($x: expr, $digits: expr) => {
        (($x * 10f64.powi($digits)) as f64).round() / 10f64.powi($digits)
    };
}

fn is_partitioned<T: Copy + PartialOrd>(xs: &[T], pivot_elem: T) -> bool {
    match xs.iter().position(|&x| x == pivot_elem) {
        Some(pos) => {
            let left = &xs[..pos];
            let right = &xs[pos..];
            if left.iter().all(|x| x < &pivot_elem) && right.iter().all(|x| x >= &pivot_elem) {
                return true;
            }
            return false;
        }
        None => panic!("Error, no pivot element has been found!"),
    }
}

fn into_notnans(xs: &[f64]) -> Vec<NotNaN<f64>> {
    xs.iter().map(|x| NotNaN::new(*x).unwrap()).collect()
}

fn ensure_partitioned(mut xs: Vec<u32>, pivot_idx: usize) -> TestResult {
    let l = xs.len();

    if l == 0 {
        TestResult::discard()
    } else if pivot_idx >= l {
        TestResult::discard()
    } else {
        let pivot_elem = xs[pivot_idx];
        partition(&mut xs, pivot_idx, 0, l);
        TestResult::from_bool(is_partitioned(&xs, pivot_elem))
    }
}

fn ensure_statistics(mut xs: Vec<u32>, mut ks: Vec<usize>) -> TestResult {
    let len_xs = xs.len();
    let len_ks = ks.len();

    if len_xs == 0 {
        TestResult::discard()
    } else if len_ks >= len_xs {
        TestResult::discard()
    } else {
        // ensure all ks indices fit into the source vector bounds
        for k in &ks {
            if k >= &len_xs {
                return TestResult::discard();
            }
        }

        let mut ys = xs.clone();
        let mut passed = true;

        let result = kth_stats_recur(&mut ys, &mut ks);

        xs.sort();

        // check that all elements found by kth_stat_recur are the same as
        // corresponding elements in sorted list
        for k in ks {
            if xs[k] != *result.get(&k).unwrap() {
                passed = true;
                break;
            }
        }

        TestResult::from_bool(passed)
    }
}

#[test]
fn test_partition() {
    quickcheck(ensure_partitioned as fn(Vec<u32>, usize) -> TestResult);
}

#[test]
fn test_kth() {
    quickcheck(ensure_statistics as fn(Vec<u32>, Vec<usize>) -> TestResult);
}

#[test]
fn test_variance() {
    let input: Vec<f64> = vec![];
    assert!(variance(input).is_err());

    let input = vec![2.75];
    assert!(variance(input).is_err());

    let input = vec![2.75, 1.75, 1.25, 0.25, 0.5, 1.25, 3.5];
    assert_eq!(round!(variance(input).unwrap(), 4), 1.3720);

    let input = vec![27.5, 30.25, 30.25, 34.5, 41.75];
    assert_eq!(round!(variance(input).unwrap(), 4), 31.0188);
}

#[test]
fn test_pvariance() {
    let input: Vec<f64> = vec![];
    assert!(pvariance(input).is_err());

    let input = vec![2.75];
    assert_eq!(round!(pvariance(input).unwrap(), 3), 0.0);

    let input = vec![0.0, 0.25, 0.25, 1.25, 1.5, 1.75, 2.75, 3.25];
    assert_eq!(round!(pvariance(input).unwrap(), 3), 1.25);

    let input = vec![27.5, 30.25, 30.25, 34.5, 41.75];
    assert_eq!(round!(pvariance(input).unwrap(), 4), 24.815);

}

#[test]
fn test_mean() {
    let input: Vec<f64> = vec![];
    assert!(mean(input).is_err());

    let input = vec![2.0];
    assert_eq!(mean(input).unwrap(), 2.0);

    let input = vec![2.0, 3.0];
    assert_eq!(mean(input).unwrap(), 2.5);

    let input = vec![2.0, -2.0, 3.0, -3.0, 4.0, -4.0];
    assert_eq!((mean(input).unwrap() as f64).round(), 0.0);
}

#[test]
fn test_median_grouped() {
    let mut converted = into_notnans(&[1.0, 2.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0, 4.0, 5.0]);
    let res = median_grouped(converted.as_mut_slice(), 1);
    assert_eq!(*res.unwrap(), 3.7);

    let mut converted = into_notnans(&[52.0, 52.0, 53.0, 54.0]);
    let res = median_grouped(converted.as_mut_slice(), 1);
    assert_eq!(*res.unwrap(), 52.5);

    let mut converted = into_notnans(&[1.0, 3.0, 3.0, 5.0, 7.0]);
    let res = median_grouped(converted.as_mut_slice(), 1);
    assert_eq!(*res.unwrap(), 3.25);

    let res = median_grouped(converted.as_mut_slice(), 2);
    assert_eq!(*res.unwrap(), 3.5);
}
