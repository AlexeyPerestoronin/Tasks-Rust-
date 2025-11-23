// https://leetcode.com/problems/rotate-list

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl From<Vec<i32>> for ListNode {
    fn from(value: Vec<i32>) -> Self {
        fn add_tail(value: &Vec<i32>, i: usize) -> Option<Box<ListNode>> {
            if value.len() == i {
                None
            } else {
                Some(Box::new(ListNode {
                    val: value[i],
                    next: add_tail(value, i + 1),
                }))
            }
        }

        ListNode {
            val: *value.get(0).expect("source vector must have some elements"),
            next: add_tail(&value, 1),
        }
    }
}

impl From<&ListNode> for Vec<i32> {
    fn from(value: &ListNode) -> Self {
        fn add_value(value: &ListNode, mut source: Vec<i32>) -> Vec<i32> {
            source.push(value.val);
            match &value.next {
                Some(next_value) => add_value(&next_value, source),
                None => source,
            }
        }

        add_value(&value, Vec::new())
    }
}

pub struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let size: i32 = (|| {
            let mut size: i32 = 0;
            let mut head_ref: &Option<Box<ListNode>> = &head;
            while head_ref.is_some() {
                size += 1;
                head_ref = &head_ref.as_ref().unwrap().next;
            }
            return size;
        })();

        if size == 0 {
            return None;
        }

        if size == 1 || size == k || k % size == 0 || k == 0 {
            return head;
        }

        let offset = if size > k {
            size - k
        } else {
            size - (k % size)
        };

        let mut old_head = head.unwrap();
        let mut new_tail = &mut old_head;
        for _ in 1..offset {
            let next = &mut new_tail.next;
            let next = next.as_mut().unwrap();
            new_tail = next;
        }

        let mut new_head = new_tail.next.take().unwrap();
        let mut old_tail = &mut new_head;
        for _ in 1..(size - offset) {
            let next = &mut old_tail.next;
            let next = next.as_mut().unwrap();
            old_tail = next;
        }

        old_tail.next = Some(old_head);

        Some(new_head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate leet_code_macro;
    use leet_code_macro::leet_code_test;

    #[leet_code_test(
        struct InputType(Vec<i32>, i32);
        struct OutputType(Vec<i32>);
        case_1_test = {InputType(vec![1,2,3,4,5], 1),  OutputType(vec![5,1,2,3,4])};
        case_2_test = {InputType(vec![1,2,3,4,5], 2),  OutputType(vec![4,5,1,2,3])};
        case_3_test = {InputType(vec![1,2,3,4,5], 3),  OutputType(vec![3,4,5,1,2])};
        case_4_test = {InputType(vec![1,2,3,4,5], 4),  OutputType(vec![2,3,4,5,1])};
        case_5_test = {InputType(vec![1,2,3,4,5], 5),  OutputType(vec![1,2,3,4,5])};
        case_6_test = {InputType(vec![1,2,3,4,5], 6),  OutputType(vec![5,1,2,3,4])};
        case_7_test = {InputType(vec![1,2,3,4,5], 7),  OutputType(vec![4,5,1,2,3])};
        case_8_test = {InputType(vec![0,1,2], 4),      OutputType(vec![2,0,1])};
        case_9_test = {InputType(vec![1], 0),          OutputType(vec![1])};
        case_a_test = {InputType(vec![1], 1),          OutputType(vec![1])};
        case_b_test = {InputType(vec![1], 99),         OutputType(vec![1])};
        case_c_test = {InputType(vec![1,2,3,4,5], 10), OutputType(vec![1,2,3,4,5])};
    )]
    fn rotate_right_test(input: InputType, result: OutputType) -> () {
        let head = Some(Box::new(ListNode::from(input.0)));
        let k = input.1;
        let head = Solution::rotate_right(head, k);
        let result_vec = match head {
            Some(head) => Vec::<i32>::from(&(*head)),
            None => Vec::<i32>::new(),
        };
        assert_eq!(result_vec, result.0);
    }

    #[test]
    fn rotate_right_none_test() -> () {
        let head: Option<Box<ListNode>> = None;
        let head = Solution::rotate_right(head, 11);
        assert_eq!(head, None);
    }
}
