// https://leetcode.com/problems/two-sum

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut res = Vec::<i32>::new();

        let mut map: HashMap<i32, usize> = HashMap::new();
        for (index1, num) in nums.into_iter().enumerate() {
            if let Some(index2) = map.get(&(target - num)) {
                res.push(index1 as i32);
                res.push(*index2 as i32);
                break;
            }

            if let Some(index2) = map.get(&num) {
                if num * 2 == target {
                    res.push(index1 as i32);
                    res.push(*index2 as i32);
                    break;
                }
            }

            map.insert(num, index1);
        }

        res.sort();
        return res;
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
        case_1_test = {InputType(vec![2, 7, 11, 15], 9), OutputType(vec![0, 1])};
        case_2_test = {InputType(vec![3, 2, 4], 6),      OutputType(vec![1, 2])};
        case_3_test = {InputType(vec![3, 3], 6),         OutputType(vec![0, 1])};
    )]
    fn two_sum_test(input: InputType, result: OutputType) -> () {
        assert_eq!(Solution::two_sum(input.0, input.1), result.0);
    }
}
