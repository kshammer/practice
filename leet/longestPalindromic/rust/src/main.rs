struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect::<Vec<_>>();
        return Self::helper(chars); 
    }
    fn helper(chars: Vec<char>) -> String {
        let mid_point:i32 = (chars.len() / 2) as i32;
        let is_odd = chars.len() % 2 == 1;
        let mut left:i32;
        let mut right:i32;
        if is_odd {
            left = mid_point - 1; 
            right = mid_point + 1; 
        } else {
            left = mid_point -1; 
            right = mid_point;     
        }
        loop {
            let left_val = match chars.get(left as usize) {
                Some(x) => *x, 
                None => break,
            };
            let right_val = match chars.get(right as usize) {
                Some(x) => *x, 
                None => break, 
            }; 
            if left_val != right_val{
                break; 
            }else {
                left = left - 1;
                right = right + 1; 
            }
        }
    
        return chars[(left + 1) as usize..right as usize].into_iter().collect();
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_babad() {
        let answer = Solution::longest_palindrome("babad".to_string());
        assert_eq!(answer, "aba");
    }

    #[test]
    fn test_cbbd() {
        let answer = Solution::longest_palindrome("cbbd".to_string());
        assert_eq!(answer, "bb");
    }
}

fn main() {
    println!("Hello, world");
}
