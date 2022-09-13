struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let is_odd = (nums1.len() + nums2.len()) % 2 == 1;
        let iterations = (nums1.len() + nums2.len()) / 2;
        let mut index = 0;
        let mut left = 0;
        let mut right = 0;
        let mut curr_val = 0;
        while index != iterations {
            let tup = Self::next(&nums1, &nums2, left, right);
            curr_val = tup.0;
            left = tup.1;
            right = tup.2; 
            index += 1;
        }
        if is_odd {
            println!("current {}", curr_val);
            return Self::next(&nums1, &nums2, left, right).0 as f64;
        } else {
            let ret = Self::next(&nums1, &nums2, left, right);
            return ((ret.0 as f64) + (curr_val as f64)) / 2.0;
        }
    }

    fn next(
        vec1: &Vec<i32>,
        vec2: &Vec<i32>,
        left_index: usize,
        right_index: usize,
    ) -> (i32, usize, usize) {
        let left_val = match vec1.get(left_index) {
            Some(x) => x,
            None => &i32::MAX,
        };
        let right_val = match vec2.get(right_index) {
            Some(x) => x,
            None => &i32::MAX,
        };

        if left_val <= right_val {
            return (left_val.clone(), left_index + 1, right_index);
        } else {
            return (right_val.clone(), left_index, right_index + 1);
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
