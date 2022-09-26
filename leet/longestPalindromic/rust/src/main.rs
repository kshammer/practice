struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect::<Vec<_>>();
        let longest = Self::helper(&chars); 
        
        let mid_point:i32 = (chars.len() / 2) as i32; 

        return Self::helper(&chars); 
    }
    fn helper(chars: &Vec<char>) -> String {
        if chars.len() == 1 {
            return chars[0].to_string();
        }
        else if chars.len() == 2 {
            if chars[0] == chars[1] {
                return chars.into_iter().collect(); 
            } else {
                return chars[0].to_string(); 
            }
        }

        let mid_point:i32 = (chars.len() / 2) as i32;
        let is_odd = chars.len() % 2 == 1;
        let mut left:i32;
        let mut right:i32;
        if is_odd {
            left = mid_point; 
            right = mid_point; 
        } else {
            left = mid_point -1; 
            right = mid_point;     
        }
        loop {
            let left_val = match chars.get((left -1) as usize) {
                Some(x) => *x, 
                None => break,
            };
            let right_val = match chars.get((right + 1) as usize) {
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
            
        return chars[left as usize..=right as usize].into_iter().collect();
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

    #[test]
    fn test_ac() {
        let answer = Solution::longest_palindrome("ac".to_string());
        assert_eq!(answer, "a");
    }
    #[test]
    fn test_bb() {
        let answer = Solution::longest_palindrome("bb".to_string());
        assert_eq!(answer, "bb");
    }

    #[test]
    fn test_a() {
        let answer = Solution::longest_palindrome("a".to_string());
        assert_eq!(answer, "a");
    }

    #[test]
    fn test_abb() {
        let answer = Solution::longest_palindrome("abb".to_string());
        assert_eq!(answer, "bb");
    }
}

fn main() {
    println!("Hello, world");
}
