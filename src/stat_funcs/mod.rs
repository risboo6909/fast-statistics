crate mod errors;

use self::errors::MyError;

use superslice::Ext;
use int_hash::IntHashMap;
use num::{Float, FromPrimitive, Num};
use ordered_float::NotNaN;
use rand::{Rng, SeedableRng, XorShiftRng};
use rayon::prelude::*;
use std::cmp::{max, min, Reverse};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Div, Mul};

// TODO: Make it configurable as function argument?
// this constant was empirically chosen to make kth_stat algorithm work well on various input
// data samples
const KTH_SORT_THRESHOLD: f64 = 0.1 / 100.0;

#[inline]
fn init_rand() -> impl FnMut(usize, usize) -> usize {
    let mut rng: XorShiftRng = XorShiftRng::from_seed(rand::thread_rng().gen());
    return move |from: usize, to: usize| -> usize {
        if from == to {
            0
        } else {
            from + (rng.next_u64() % (to as u64 - from as u64)) as usize
        }
    };
}

crate fn mode<T: Eq + Ord + Clone + Hash + Debug>(xs: Vec<T>) -> Result<T, MyError> {
    if xs.len() == 0 {
        return Err(MyError::NoModeEmptyData);
    }

    // create mapping from elements to their frequencies
    let pairs = xs.into_iter().fold(HashMap::new(), |mut acc, e| {
        (*acc.entry(e).or_insert(0)) += 1;
        acc
    });

    // sort modes by their frequencies
    let mut tmp = pairs.into_iter().collect::<Vec<(T, u64)>>();
    tmp.sort_by_key(|x| Reverse(x.1));

    // first element must be mode element
    let (mode, mode_val) = tmp[0].clone();

    // count number of elements with the same frequency as the mode element
    let modes = tmp.into_iter().take_while(|x| x.1 == mode_val).count();

    match modes {
        // one unique mode found
        1 => Ok(mode),
        // many modes with equal frequencies found
        _ => Err(MyError::NoUniqueMode { modes: modes }),
    }
}

crate fn harmonic_mean<T>(xs: Vec<T>) -> Result<T, MyError>
where
    T: Num + PartialOrd + Float,
{
    if xs.len() == 0 {
        return Err(MyError::HarmonicNoDataPoints);
    }

    let result = xs.into_iter().try_fold((T::zero(), T::zero()), |acc, e| {
        if e >= T::zero() {
            Some((acc.0 + e.recip(), acc.1 + T::one()))
        } else {
            None
        }
    });

    match result {
        Some((sum, len)) => Ok(len / sum),
        None => Err(MyError::HarmonicNegatives),
    }
}

#[inline]
fn get_median_pair<'a, T: 'a>(r: &'a IntHashMap<usize, T>) -> (&'a T, &'a T) {
    let v = r.values().collect::<Vec<&T>>();
    (v[0], v[1])
}

crate fn median<T>(xs: &mut [T]) -> Result<T, MyError>
where
    T: Copy + PartialOrd + Num + Add + Send + Debug + From<f32>,
{
    let n = xs.len();

    if n == 0 {
        return Err(MyError::NoMedianEmptyData);
    }

    let med_idx = (0.5 * n as f64) as usize;

    if n % 2 == 0 {
        let r = kth_stats_recur(xs, &mut [med_idx - 1, med_idx]);
        let (a, b) = get_median_pair(&r);
        Ok((*a + *b) / T::from(2.0))
    } else {
        kth_stat(xs, med_idx)
    }
}

fn median_low_high<T>(xs: &mut [T], f: fn(T, T) -> T) -> Result<T, MyError>
where
    T: Copy + Ord + Send + Debug,
{
    let xs_len = xs.len();
    let med_idx = (0.5 * xs_len as f64) as usize;

    if xs_len % 2 == 0 {
        let r = kth_stats_recur(xs, &mut [med_idx - 1, med_idx]);
        let (a, b) = get_median_pair(&r);
        Ok(f(*a, *b))
    } else {
        kth_stat(xs, med_idx)
    }
}

crate fn median_low<T: Copy + Ord + Send + Debug>(ys: &mut [T]) -> Result<T, MyError> {
    // Helper function
    median_low_high(ys, min)
}

crate fn median_high<T: Copy + Ord + Send + Debug>(ys: &mut [T]) -> Result<T, MyError> {
    // Helper function
    median_low_high(ys, max)
}

/// Compute median of grouped continuous data
///
/// median = L + interval * (N / 2 - CF) / F
///
/// L = lower limit of the median interval
/// N = total number of data points
/// CF = number of data points below the median interval
/// F = number of data points in the median interval
///
/// see https://www.geeksforgeeks.org/python-statistics-median_grouped/ for explanation
crate fn median_grouped(xs: &mut [NotNaN<f64>], interval: usize) -> Result<NotNaN<f64>, MyError> {
    xs.sort();
    let n = xs.len();

    if n == 0 {
        return Err(MyError::NoMedianEmptyData);
    } else if n == 1 {
        return Ok(xs[0]);
    }

    let x = xs[n / 2];

    let lower_limit = x - 0.5 * interval as f64;

    let l1 = xs.lower_bound(&x);
    let l2 = xs.upper_bound(&x) - 1;

    let cf = l1;
    let f = (l2 - l1 + 1) as f64;

    Ok(lower_limit + interval as f64 * ((0.5 * (n as f64) - cf as f64) / f) as f64)
}

/// Naive implementations of variance/mean computation suffer from a lack of precision
/// therefor more advanced and much more accurate technique will be used, see:
///
/// https://math.stackexchange.com/questions/20593/calculate-variance-from-a-stream-of-sample-values
/// https://www.johndcook.com/blog/standard_deviation/
///
/// for details
#[inline]
#[allow(unused_mut)]
fn running_stat<T>() -> impl FnMut(T) -> (T, T)
where
    T: Num
        + Copy
        + FromPrimitive
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Add<T, Output = T>,
{
    let mut m_n = 1;

    let mut old_m = T::zero();
    let mut old_s = T::zero();

    return move |x: T| {
        let new_m;
        let new_s;

        if m_n == 1 {
            new_s = T::zero();
            old_s = T::zero();

            old_m = x;
            new_m = x;
        } else {
            let common_diff = x - old_m;

            new_m = old_m + common_diff / T::from_usize(m_n).unwrap();
            new_s = old_s + common_diff * (x - new_m);

            old_m = new_m;
            old_s = new_s;
        }

        m_n += 1;

        (new_m, new_s)
    };
}

/// Return the sample variance of input data
crate fn variance<T>(xs: Vec<T>) -> Result<T, MyError>
where
    T: Float
        + FromPrimitive
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Add<T, Output = T>,
{
    if xs.len() < 2 {
        Err(MyError::NoEnoughDataForVariance)
    } else {
        let mut push_one = running_stat();
        let mut res = (T::zero(), T::zero());

        for x in xs.iter() {
            res = push_one(*x);
        }

        Ok(res.1 / T::from_usize(xs.len() - 1).unwrap())
    }
}

/// Return the population variance of input data
crate fn pvariance<T>(xs: Vec<T>) -> Result<T, MyError>
where
    T: Float
        + FromPrimitive
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Add<T, Output = T>,
{
    if xs.len() < 1 {
        Err(MyError::NoEnoughDataForPopulationVariance)
    } else {
        let mut push_one = running_stat();
        let mut res = (T::zero(), T::zero());

        for x in xs.iter() {
            res = push_one(*x);
        }

        Ok(res.1 / T::from_usize(xs.len()).unwrap())
    }
}

crate fn stdev<T>(xs: Vec<T>) -> Result<T, MyError>
where
    T: Float
        + FromPrimitive
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Add<T, Output = T>,
{
    let res = variance(xs)?;
    // variance can't be a negative value no additional checks needed
    Ok(res.sqrt())
}

crate fn pstdev<T>(xs: Vec<T>) -> Result<T, MyError>
where
    T: Float
        + FromPrimitive
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Add<T, Output = T>,
{
    let res = pvariance(xs)?;
    // pvariance can't be a negative value no additional checks needed
    Ok(res.sqrt())
}

crate fn mean<T>(xs: Vec<T>) -> Result<T, MyError>
where
    T: Num
        + Copy
        + FromPrimitive
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Add<T, Output = T>,
{
    if xs.len() < 1 {
        Err(MyError::NoEnoughDataForMean)
    } else {
        let mut push_one = running_stat();
        let mut res = (T::zero(), T::zero());

        for x in xs.iter() {
            res = push_one(*x);
        }

        Ok(res.0)
    }
}

/// Partition input slice xs in-place, such that elements smaller than the pivot are at the
/// left side and elements bigger than the pivot are at the right side.
///
/// # Example
/// ```
/// let xs = &mut [1, 5, 6, 2, 3, 7, 10, 9, 4, 8];
/// let l = xs.len();
/// partition(xs, 1, 0, l);
///
/// println!("{:?}", xs);
/// ```
fn partition<T: Copy + PartialOrd>(
    xs: &mut [T],
    pivot_idx: usize,
    start: usize,
    end: usize,
) -> usize {
    let pivot_elem = xs[pivot_idx];

    xs.swap(end - 1, pivot_idx);

    let (mut i, mut j) = (start, start);

    loop {
        if j >= end - 1 {
            break;
        }

        if xs[i] < pivot_elem {
            i += 1;
            j = usize::max(i, j);
        } else {
            if xs[j] >= pivot_elem {
                j += 1;
            } else {
                xs.swap(i, j);
                i += 1;
            }
        }
    }

    xs.swap(i, j);

    i
}

fn kth_stat_helper<T: Copy + PartialOrd + Send + Debug>(
    rand_range: &mut impl FnMut(usize, usize) -> usize,
    xs: &mut [T],
    ks: &mut Vec<usize>,
    left: usize,
    right: usize,
    need_sort: bool,
) -> IntHashMap<usize, T> {
    let empty_hash = IntHashMap::default();

    if left >= right || ks.len() == 0 {
        return empty_hash;
    }

    if need_sort {
        // sort selected array part and choose elements we need
        let ys = &mut xs[left..right];
        ys.par_sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let mut found = empty_hash;

        for elem in ks {
            found.insert(*elem, ys[*elem - left]);
        }

        return found;
    }

    // choose random pivot point
    let pivot_idx = rand_range(left, right);

    // partition an array into two halves, one consists of all elements less than
    // the pivot and another one consists of all elements bigger than the pivot
    let real_idx = partition(xs, pivot_idx, left, right);

    // assess how good an array was partitioned by analyzing sizes of its left and right half
    let left_len = real_idx - left;
    let right_len = right - real_idx;

    // compare two halves relative size
    let need_sort =
        if left_len >= right_len && (right_len as f64 / left_len as f64 <= KTH_SORT_THRESHOLD) {
            true
        } else if left_len as f64 / right_len as f64 <= KTH_SORT_THRESHOLD {
            true
        } else {
            false
        };

    let ks_len = ks.len();
    let mut found = empty_hash;

    // tricky part, ks - is a sorted array of statistics that we want to
    // find, for example [10, 30, 50, 70, 99, 150], then we will use binary search to
    // figure out position of pivot element in the ks list
    let k_idx = match ks.binary_search(&real_idx) {
        Ok(k_idx) => {
            found.insert(ks.remove(k_idx), xs[real_idx]);
            k_idx
        }
        Err(k_idx) => k_idx,
    };

    if k_idx > 0 && k_idx < ks_len {
        // if index of pivot element was somewhere inside of ks list, we need 2 recursive calls
        // one to find all the elements lesser than the pivot element and another one to find
        // all the elements bigger than the pivot element
        let (ks_left, ks_right) = ks.split_at(k_idx);

        found.extend(kth_stat_helper(
            rand_range,
            xs,
            &mut ks_left.to_vec(),
            left,
            real_idx,
            need_sort,
        ));
        found.extend(kth_stat_helper(
            rand_range,
            xs,
            &mut ks_right.to_vec(),
            real_idx + 1,
            right,
            need_sort,
        ));
    } else if k_idx == 0 {
        // if the leftmost element of ks was found only one recursive call is required, because
        // it is guaranteed that there are no elements with the position smaller than k_idx
        found.extend(kth_stat_helper(
            rand_range,
            xs,
            ks,
            real_idx + 1,
            right,
            need_sort,
        ));
    } else if k_idx == ks_len {
        // if the rightmost element of ks was found only one recursive call is required because
        // it is guaranteed that there are no elements with the position bigger than k_idx
        found.extend(kth_stat_helper(
            rand_range, xs, ks, left, real_idx, need_sort,
        ));
    };

    found
}

/// Kth statistic works in amortized linear time O(n), the worst case will still be O(n^2).
///
/// To avoid quadratic time in the worst case, we analyze sizes of two halves which were
/// produced by the algorithm on each step and if one of the halves is much bigger than another
/// one -- give up and use sort
crate fn kth_stats_recur<T: Copy + PartialOrd + Send + Debug>(
    xs: &mut [T],
    ks: &mut [usize],
) -> IntHashMap<usize, T> {
    let xs_len = xs.len();
    let ks_vec = &mut ks.to_vec();

    ks_vec.sort_unstable();
    ks_vec.dedup();

    let mut rand_range = init_rand();

    kth_stat_helper(&mut rand_range, xs, ks_vec, 0, xs_len, false)
}

crate fn kth_stat<T: Copy + PartialOrd + Send + Debug>(
    xs: &mut [T],
    k: usize,
) -> Result<T, MyError> {
    Ok(*kth_stats_recur(xs, &mut [k]).get(&k).unwrap())
}

#[cfg(test)]
mod tests {
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

}
