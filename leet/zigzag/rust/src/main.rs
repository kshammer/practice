struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize; 

        if num_rows < 2 {
            return s; 
        }

        let mut strings = vec![String::new(); num_rows];

        let mut i = 0; 
        let mut down = true; 

        for c in s.chars() {
            strings[i].push(c); 
            i = if down { i + 1 } else { i -1 }; 
            down = down == ( i > 0 && i < num_rows -1);
        }

        strings.concat()


    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_3_rows() {
        let answer = Solution::convert("PAYPALISHIRING".to_string(), 3);
        assert_eq!("PAHNAPLSIIGYIR", answer);
    }

    #[test]
    fn test_4_rows() {
        let answer = Solution::convert("PAYPALISHIRING".to_string(), 4);
        assert_eq!("PINALSIGYAHRPI", answer);
    }

    #[test]
    fn test_1_rows() {
        let answer = Solution::convert("A".to_string(), 1);
        assert_eq!("A", answer);
    }
}

fn main() {
    println!("Hello, world!");
}
