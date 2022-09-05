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
        let mut one = l1.unwrap();
        let mut two = l2.unwrap();
        let mut root = ListNode::new(0);

        let mut res = Solution::make_node(one.val + two.val);
        root.next.get_or_insert(Box::new(res.0));

        let mut curr = &mut root.next;

        while one.next.is_some() || two.next.is_some() {
            match curr {
                None => break,
                Some(now) => {
                    one = one.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
                    two = two.next.or(Some(Box::new(ListNode::new(0)))).unwrap();

                    res = Solution::make_node(one.val + two.val + res.1);

                    now.next.get_or_insert(Box::new(res.0));
                    curr = &mut now.next;
                }
            }
        }

        if res.1 > 0 {
            if let Some(now) = curr {
                now.next.get_or_insert(Box::new(ListNode::new(res.1)));
            }
        }

        root.next
    }

    fn make_node(mut result: i32) -> (ListNode, i32) {
        let single;
        if result > 9 {
            single = 1;
            result = result - 10;
        } else {
            single = 0;
        }
        (ListNode::new(result), single)
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
        assert_eq!(3, n.val);

        n = n.next.unwrap();
        assert_eq!(1, n.val);
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

}

fn main() {
    println!("Hello, World");
}
