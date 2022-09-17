struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn is_palindrome(s: &[u8]) ->bool {
            s.iter().zip(s.iter().rev()).all(|(l, r)| l == r)
        }

        for size in (1..=s.len()).rev() {
            match s.as_bytes().windows(size).find(|substr| is_palindrome(substr)){
                Some(pal) => return String::from_utf8(pal.to_vec()).unwrap(),
                None => continue,
            }
        }
        // None 
        String::from("")
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_babad() {
        let answer = Solution::longest_palindrome("babad".to_string());
        assert_eq!(answer, "bab");
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
