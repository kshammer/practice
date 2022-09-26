use std::str::FromStr; 

struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let negative = if x < 0 { true } else { false };
        if negative {
            x = x * -1;
        }
        let as_str = x.to_string().chars().rev().collect::<String>();
        let mut cool = i32::from_str(&as_str).unwrap_or(0);
        if negative {
            cool = cool * -1; 
        } 
    
        cool  
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_one() {
        let answer = Solution::reverse(123);
        assert_eq!(321, answer);
    }

    #[test]
    fn test_negative() {
        let answer = Solution::reverse(-321);
        assert_eq!(-123, answer);
    }

    #[test]
    fn test_zero() {
        let answer = Solution::reverse(120);
        assert_eq!(21, answer);
    }

    #[test]
    fn test_overflow() {
        let answer = Solution::reverse(1534236469);
        assert_eq!(0, answer);
    }
}

fn main() {
    println!("Hello, world!");
}
