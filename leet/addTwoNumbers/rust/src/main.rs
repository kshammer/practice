#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        }
        let mut carry = 0;

        let first = Box::new(ListNode::new(0));

        while l1.is_some() || l2.is_some() {
            let val1 = match l1 {
                Some(ref x) => x.val,
                None => 0,
            };
            let val2 = match l2 {
                Some(ref x) => x.val,
                None => 0,
            };
            let mut sum = val1 + val2 + carry;
            if sum >= 10 {
                sum = sum % 10;
                carry = 1;
            }
        }

        return None;
    }

    fn helper(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        mut carry: i32,
    ) -> (Option<Box<ListNode>>, i32) {
        let val1 = match l1 {
            Some(ref x) => x.val,
            None => 0,
        };
        let val2 = match l2 {
            Some(ref x) => x.val,
            None => 0,
        };
        let mut sum = val1 + val2 + carry;
        if sum >= 10 {
            sum = sum % 10;
            carry = 1;
        }

        // check if sum and carry are not zero =) 

        return (None, carry);
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn no_carry_test() {
        let answer = Solution::add_two_numbers(
            Some(Box::new(ListNode {
                next: Some(Box::new(ListNode::new(1))),
                val: 2,
            })),
            Some(Box::new(ListNode::new(1))),
        );
        let mut n = answer.unwrap();
        assert_eq!(2, n.val);

        n = n.next.unwrap();
        assert_eq!(3, n.val);
    }

    #[test]
    fn carry_test() {
        let answer = Solution::add_two_numbers(
            Some(Box::new(ListNode {
                next: Some(Box::new(ListNode::new(3))),
                val: 9,
            })),
            Some(Box::new(ListNode::new(1))),
        );
        let mut n = answer.unwrap();
        assert_eq!(0, n.val);

        n = n.next.unwrap();
        assert_eq!(4, n.val);
    }

    #[test]
    fn carry_end_test() {
        let answer = Solution::add_two_numbers(
            Some(Box::new(ListNode::new(9))),
            Some(Box::new(ListNode::new(1))),
        );
        let mut n = answer.unwrap();
        assert_eq!(0, n.val);

        n = n.next.unwrap();
        assert_eq!(1, n.val);
    }

    #[test]
    fn carry_twice_test() {
        let answer = Solution::add_two_numbers(
            Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(7))),
            })),
            Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode::new(2))),
            })),
        );
        let mut n = answer.unwrap();
        assert_eq!(2, n.val);

        n = n.next.unwrap();
        assert_eq!(0, n.val);

        n = n.next.unwrap();
        assert_eq!(1, n.val);
    }

    #[test]
    fn empty_l2_lists_test() {
        let answer = Solution::add_two_numbers(
            Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode::new(3))),
            })),
            None,
        );
        let mut n = answer.unwrap();
        assert_eq!(9, n.val);

        n = n.next.unwrap();
        assert_eq!(3, n.val);
    }

    #[test]
    fn empty_l1_lists_test() {
        let answer = Solution::add_two_numbers(
            None,
            Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode::new(3))),
            })),
        );
        let mut n = answer.unwrap();
        assert_eq!(9, n.val);

        n = n.next.unwrap();
        assert_eq!(3, n.val);
    }

    #[test]
    fn both_empty_test() {
        let answer = Solution::add_two_numbers(None, None);
        assert!(answer.is_none());
    }
}

fn main() {
    println!("Hello, World");
}
