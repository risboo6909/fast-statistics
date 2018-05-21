extern crate rayon;
extern crate rand;
extern crate num;

use stat_funcs::rayon::prelude::*;
use self::rand::Rng;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, Div};
use std::collections::{HashMap, BTreeMap};
use self::num::{Zero, One, Num};
use super::errors::MyError;


#[inline]
fn rand_range(from: usize, to: usize) -> usize {
    if from == to {
        0
    } else {
        let mut rng = rand::thread_rng();
        from + (rng.next_u64() % (to as u64 - from as u64)) as usize
    }
}


/// Uses rayon to compute mode of a sequence in parallel
pub fn mode<T: Send + Eq + Ord + Debug>(xs: Vec<T>) -> Result<T, MyError> {

    let pairs  = xs.into_par_iter()
        .fold(|| BTreeMap::new(), |mut acc, e| {
            (*acc.entry(e).or_insert(0)) += 1;
            acc
        })
        .reduce(|| BTreeMap::new(), |mut acc, part| {
            for (k, v) in part.into_iter() {
                (*acc.entry(k).or_insert(0)) += v;
            };
            acc
        });

    let best = pairs.into_par_iter().max_by_key(|pair| pair.1).unwrap();

    Ok(best.0)

}

pub fn avg_num<T>(xs: Vec<T>) -> Result<<T as Div>::Output, MyError>
    where for<'a> T: Add + Send + Zero + One + PartialEq + Div {

    let (sum, len) = xs.into_par_iter()
        .fold(|| (T::zero(), T::zero()), |acc, y|
            (acc.0 + y, acc.1 + T::one()))
        .reduce(|| (T::zero(), T::zero()), |acc, e|
            (acc.0 + e.0, acc.1 + e.1));

    if len == T::zero() {
        return Err(MyError::ZeroDivisionError);
    }

    Ok(sum / len)
}

pub fn harmonic_mean(xs: Vec<f64>) -> Result<f64, MyError> {

    let (sum, len) = xs.into_par_iter()
        .fold(|| (0.0, 0.0), |acc, y|
            (acc.0 + y.recip(), acc.1 + 1.0))
        .reduce(|| (0.0, 0.0), |acc, e|
            (acc.0 + e.0, acc.1 + e.1));
//
//    if len == 0.0 {
//        return Err(InternalStatisticsError{msg: ''});
//    }

    Ok(len / sum)
}

#[inline]
fn get_two_med<'a, T: 'a>(r: &'a HashMap<usize, T>) -> (&'a T, &'a T) {
    let v = r.values().collect::<Vec<&T>>();
    (v[0], v[1])
}

#[inline]
fn partial_min<T: PartialOrd + Copy>(a: T, b: T) -> Option<T> {
    match a.partial_cmp(&b) {
        Some(Ordering::Less) | Some(Ordering::Equal) => {
            Some(a)
        },
        Some(Ordering::Greater) => {
            Some(b)
        },
        None => None
    }
}

#[inline]
fn partial_max<T: PartialOrd + Copy>(a: T, b: T) -> Option<T> {
    match a.partial_cmp(&b) {
        Some(Ordering::Less) | Some(Ordering::Equal) => {
            Some(b)
        },
        Some(Ordering::Greater) => {
            Some(a)
        },
        None => None
    }
}

pub fn median<T>(xs: &mut [T]) -> Result<T, MyError> where T: Copy + PartialOrd + Num + Add + Debug {

    let xs_len = xs.len();
    let med_idx = (xs_len as f64 / 2.0) as usize;

    if xs_len % 2 == 0 {
        let r = kth_stats_recur(xs, &mut [med_idx - 1, med_idx]);
        let (a, b) = get_two_med(&r);
        let two = T::one() + T::one();
        Ok((*a + *b) / two)
    } else {
        kth_stat(xs, med_idx)
    }

}

pub fn median_low<T>(xs: &mut[T]) -> Result<T, MyError> where T: Copy + PartialOrd + Num + Add + Debug {

    let xs_len = xs.len();
    let med_idx = (xs_len as f64 / 2.0) as usize;
    if xs_len % 2 == 0 {
        let r = kth_stats_recur(xs, &mut [med_idx - 1, med_idx]);
        let (a, b) = get_two_med(&r);
        Ok(partial_min(*a, *b).unwrap())
    } else {
        kth_stat(xs, med_idx)
    }
}

pub fn median_high<T>(xs: &mut[T]) -> Result<T, MyError> where T: Copy + PartialOrd + Num + Add + Debug {

    let xs_len = xs.len();
    let med_idx = (xs_len as f64 / 2.0) as usize;

    if xs_len % 2 == 0 {
        let r = kth_stats_recur(xs, &mut [med_idx - 1, med_idx]);
        let (a, b) = get_two_med(&r);
        Ok(partial_max(*a, *b).unwrap())
    } else {
        kth_stat(xs, med_idx)
    }
}

pub fn median_group(xs: &mut[f64]) -> Result<f64, MyError> {
    // TODO
    // this function works with floating-point number only
    Ok(1.0)
}

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
fn partition<T: Copy + PartialOrd>(xs: &mut[T], pivot_idx: usize, start: usize, end: usize) -> usize {

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
    // the pivot and another one consists of all elements bigger than the pivot
    let real_idx = partition(xs, pivot_idx, left, right);

    let ks_len = ks.len();
    let mut found = HashMap::new();

    // tricky part, ks - is a sorted array of statistics that we want to
    // find, for example [10, 30, 50, 70, 99], then we will use binary search to
    // figure out position of pivot element in the ks list
    let k_idx = match ks.binary_search(&real_idx) {
        Ok(k_idx) => {
            found.insert(ks.remove(k_idx), xs[real_idx]);
            k_idx
        },
        Err(k_idx) => {
            k_idx
        }
    };

    if k_idx > 0 && k_idx < ks_len {

        // if index of pivot element was in the middle of ks list, we need 2 recursive calls
        // one to find all elements lesser than the pivot element and another one to find
        // all elements bigger than the pivot element

        let (ks_left, ks_right) = ks.split_at(k_idx);
        found.extend(kth_stat_helper(xs, &mut ks_left.to_vec(), left, real_idx));
        found.extend(kth_stat_helper(xs, &mut ks_right.to_vec(), real_idx + 1, right));

    } else if k_idx == 0 {
        // if the leftmost element of ks was found only one recursive call is required, because
        // it is guaranteed that no elements with smaller than k_idx position are required
        found.extend(kth_stat_helper(xs, ks, real_idx + 1, right));

    } else if k_idx == ks_len {
        // if the rightmost element of ks was found only one recursive call is required because
        // it is guaranteed that no elements with bigger than k_idx position are required
        found.extend(kth_stat_helper(xs, ks, left, real_idx));
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

/// Kth statistic works in amortized linear time O(n), the worst
/// case will still be O(n^2).
///
/// To avoid quadratic time in the worst case, after number (N)
/// of steps if an algorithm still didn't finish its execution
/// try to switch to trivial heapsort and get kth element from sorted
/// list. This will improve worst-case time to O(nlogn)
pub fn kth_stat<T: Copy + PartialOrd + Debug>(xs: &mut [T], k: usize) -> Result<T, MyError> {
    Ok(*kth_stats_recur(xs, &mut [k]).get(&k).unwrap())
}


#[cfg(test)]
mod tests {
    extern crate quickcheck;

    use stat_funcs::{partition, kth_stats_recur, rand_range, median};
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
        quickcheck(ensure_partitioned as fn (Vec<u32>, usize) -> TestResult);
    }

    #[test]
    fn test_kth() {
        quickcheck(ensure_statistics as fn (Vec<u32>, Vec<usize>) -> TestResult);
    }

    #[test]
    fn test_kth_recur() {
        let result = median(&mut [1.0,3.0,4.0,7.0]);
        println!("{:?}", result);
    }

}
