use std::cmp;
struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut a, mut b) = (nums1.clone(), nums2.clone());
        let total = nums1.len() as i32 + nums2.len() as i32;
        let half = total / 2;

        if b.len() < a.len() {
            (a, b) = (b, a);
        }

        let mut l = 0;
        let mut r = a.len() as i32 - 1;
        let mut i: i32;
        let mut j: i32;

        loop {
            i = if (l + r) == 0 { 0} else if (l + r) == -1{ -1} else { (l + r) / 2 }; // A  

            j = half - i - 2; // B

            let a_left = if i >= 0 { a[i as usize] } else { i32::MIN };
            let a_right = if (i + 1) < a.len() as i32 {
                a[(i + 1) as usize]
            } else {
                i32::MAX
            };
            let b_left = if j >= 0 { b[j as usize] } else { i32::MIN };
            let b_right = if (j + 1) < b.len() as i32 {
                b[(j + 1) as usize]
            } else {
                i32::MAX
            };
            if a_left <= b_right && b_left <= a_right {
                if total % 2 == 1 {
                    return cmp::min(a_right, b_right).clone() as f64;
                } else {
                    return (cmp::max(a_left, b_left) + cmp::min(a_right, b_right)) as f64 / 2.0;
                }
            } else if a_left > b_right {
                r = i - 1;
                println!("r {}, l {}", r, l); 
            } else {
                l = i + 1;
            }
        }
    }
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn test_odd() {
        let answer = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);

        assert_eq!(2.0, answer);
    }

    #[test]
    fn test_even() {
        let answer = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(2.5, answer);
    }
}

fn main() {
    println!("Hello, world!");
}
