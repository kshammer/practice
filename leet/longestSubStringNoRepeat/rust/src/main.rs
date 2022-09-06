struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest: i32 = 0;
        let mut left = 0;
        let mut right = 0;
        let char_vec: Vec<char> = s.chars().collect();
        while right + 1 != char_vec.len() {
            let curr = &char_vec[left..=right];
            let next = char_vec[right + 1];
            println!("{:?}", curr);
            if curr.contains(&next) {
                longest = if longest > char_vec[left..=right].len().try_into().unwrap() {
                    longest
                } else {
                    char_vec[left..=right].len().try_into().unwrap()
                };
                left += 1;
                right = left;
            }else{right+=1};
        }

        longest
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let answer = Solution::length_of_longest_substring(String::from("abcabcbb"));

        assert_eq!(3, answer);
    }

    #[test]
    fn test_2() {
        let answer = Solution::length_of_longest_substring(String::from("bbbbb"));
        assert_eq!(1, answer);
    }

    #[test]
    fn test_3() {
        let answer = Solution::length_of_longest_substring(String::from("pwwkew"));
        assert_eq!(3, answer);
    }
}

fn main() {
    println!("Hello,world");
}
