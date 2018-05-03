extern crate rayon;
extern crate rand;
extern crate num;

use stat_funcs::rayon::prelude::*;
use self::rand::Rng;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Div};
use std::collections::HashMap;
use self::num::{Zero, One};
use super::errors::MyError;


const FULL_SCAN_THRESHOLD_MAX: f64 = 10.0;


#[inline]
fn rand_range(from: usize, to: usize) -> usize {
    if from == to {
        0
    } else {
        let mut rng = rand::thread_rng();
        from + (rng.next_u64() % (to as u64 - from as u64)) as usize
    }
}

pub fn avg_num<T>(xs: Vec<T>) -> Result<<T as Div>::Output, MyError>
    where for<'a> T: Add + Send + Zero + One + PartialEq + Div {

    let (sum, len) = xs.into_par_iter()
        .fold(|| (T::zero(), T::zero()), |acc, y|
            (acc.0 + y, acc.1 + T::one()))
        .reduce(|| (T::zero(), T::zero()), |acc, e|
            (acc.0 + e.0, acc.1 + e.1));

    if len == T::zero() {
        return Err(MyError::DivisionByZero);
    }

    Ok(sum / len)
}

pub fn harmonic_mean(xs: Vec<f64>) -> Result<f64, MyError> {

    let (sum, len) = xs.into_par_iter()
        .fold(|| (0.0, 0.0), |acc, y|
            (acc.0 + y.recip(), acc.1 + 1.0))
        .reduce(|| (0.0, 0.0), |acc, e|
            (acc.0 + e.0, acc.1 + e.1));

    if len == 0.0 {
        return Err(MyError::DivisionByZero);
    }

    Ok(len / sum)
}

fn partition<T: Copy + PartialOrd>(xs: &mut[T], pivot_idx: usize, start: usize, end: usize) -> usize {

    /// Partition an input slice xs in-place, such that elements smaller
    /// than the pivot are at the left side and elements bigger than the pivot are
    /// at the right side.
    ///
    /// # Example
    /// ```
    /// let xs = &mut [1, 5, 6, 2, 3, 7, 10, 9, 4, 8];
    /// let l = xs.len();
    /// partition(xs, 1, 0, l);
    ///
    /// println!("{:?}", xs);
    /// ```

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


fn kth_stat_helper<T: Copy + PartialOrd + Debug>(xs: &mut[T], ks: &mut Vec<usize>,
                                                 left: usize, right: usize) -> HashMap<usize, T>
{

    if left >= right || ks.len() == 0 {
        return HashMap::new();
    }

    // choose random pivot point
    let pivot_idx = rand_range(left, right);

    // partition an array into two halves, one consists of all elements less than
    // pivot and another one consists of all elements bigger than the pivot
    let real_idx = partition(xs, pivot_idx, left, right);

    let ks_len = ks.len();
    let mut found = HashMap::new();

    let k_idx = match(ks.binary_search(&real_idx)) {
        Ok(k_idx) => {
            found.insert(ks.remove(k_idx), xs[real_idx]);
            k_idx
        },
        Err(k_idx) => {
            k_idx
        }
    };

    if k_idx > 0 && k_idx < ks_len {
        let (ks_left, ks_right) = ks.split_at(k_idx);

        let from_left = kth_stat_helper(xs, &mut ks_left.to_vec(),
                                                                 left, real_idx);

        let from_right = kth_stat_helper(xs, &mut ks_right.to_vec(),
                                                                  real_idx + 1, right);

        found.extend(from_left);
        found.extend(from_right);

    } else if k_idx == 0 {
        let from_right = kth_stat_helper(xs, ks, real_idx + 1, right);
        found.extend(from_right);

    } else if k_idx == ks_len {
        let from_left = kth_stat_helper(xs, ks, left, real_idx);
        found.extend(from_left);
    };

    found

}

pub fn kth_stats_recur<T: Copy + PartialOrd + Debug>(xs: &mut [T], ks: &mut [usize]) ->
                                                                            HashMap<usize, T> {

    let xs_len = xs.len();
    let ks_vec = &mut ks.to_vec();

    ks_vec.sort_unstable();
    ks_vec.dedup();

    kth_stat_helper(xs, ks_vec, 0, xs_len)
}

pub fn kth_stat<T: Copy + PartialOrd + Debug>(xs: &mut [T], k: usize) {

    /// Kth statistic works in amortized linear time O(n), the worst
    /// case will still be O(n^2).
    ///
    /// To avoid quadratic time in the worst case, after number (N)
    /// of steps if an algorithm still didn't finish its execution
    /// try to switch to trivial heapsort and get kth element from sorted
    /// list. This will improve worst-case time to O(nlogn)


}


#[cfg(test)]
mod tests {
    extern crate quickcheck;

    use stat_funcs::{partition, kth_stats_recur, rand_range};
    use self::quickcheck::{quickcheck, TestResult};

    fn is_partitioned<T: Copy + PartialOrd>(xs: &[T], pivot_elem: T) -> bool {
        match xs.iter().position(|&x| x == pivot_elem) {
            Some(pos) => {
                let left = &xs[..pos];
                let right = &xs[pos..];
                if left.iter().all(|x| x < &pivot_elem) &&
                    right.iter().all(|x| x >= &pivot_elem) {
                    return true;
                }
                return false;
            },
            None => panic!("Error, no pivot element has been found!")
        }
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

    fn ensure_statistics(mut xs: Vec<u32>, k: usize) -> TestResult {
        let l = xs.len();
        if l == 0 {
            TestResult::discard()
        } else if k >= l {
            TestResult::discard()
        } else {
            let mut ys = xs.clone();
            xs.sort();
            TestResult::from_bool(xs[k] == *kth_stats_recur(&mut ys, &mut [k])
                                              .get(&k)
                                              .unwrap())
        }
    }

    #[test]
    fn test_partition() {
        quickcheck(ensure_partitioned as fn (Vec<u32>, usize) -> TestResult);
    }

    #[test]
    fn test_kth() {
        quickcheck(ensure_statistics as fn (Vec<u32>, usize) -> TestResult);
    }

    #[test]
    fn test_kth_recur() {
        let result = kth_stats_recur(&mut [3,1,2,4,6,5,8,7], &mut [5, 7]);
        println!("{:?}", result);
    }

}
