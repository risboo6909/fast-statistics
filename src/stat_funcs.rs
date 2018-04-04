extern crate rayon;
extern crate rand;

use stat_funcs::rayon::prelude::*;
use self::rand::Rng;

#[inline]
fn rand_range(from: usize, to: usize) -> usize {
    let mut rng = rand::thread_rng();
    from + (rng.next_u64() % (to as u64 - from as u64)) as usize
}

fn partition<T: Copy + PartialOrd>(xs: &mut[T], pivot_idx: usize, start: usize, end: usize) {

    /// Partition an input slice xs in-place
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

}

pub fn kth_stat<T: Copy + PartialOrd>(xs: &mut [T]) -> T {

    // Kth statistic works in amortized linear time O(n), the worst
    // case will still be O(nlogn)

    let l = xs.len();

    let pivot_idx = rand_range(0, l);
    partition(xs, pivot_idx, 0, l);

    xs[0]

}


#[cfg(test)]
mod tests {
    extern crate quickcheck;

    use stat_funcs::{partition, rand_range};
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

    #[test]
    fn test_partition() {
        quickcheck(ensure_partitioned as fn (Vec<u32>, usize) -> TestResult);
    }

}