// https://leetcode.com/problems/plus-one

pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut remainder = 1i32;
        for digit in digits.iter_mut().rev() {
            if remainder == 1 {
                if *digit < 9 {
                    *digit += 1;
                    remainder = 0;
                } else {
                    *digit = 0;
                }
            } else {
                break;
            }
        }

        if remainder == 1 {
            digits.insert(0, 1);
        }
        return digits;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate leet_code_macro;
    use leet_code_macro::leet_code_test;

    #[leet_code_test(
        struct InputType(Vec<i32>);
        struct OutputType(Vec<i32>);
        case_1_test = {InputType(vec![1,2,3]),   OutputType(vec![1,2,4])};
        case_2_test = {InputType(vec![4,3,2,1]), OutputType(vec![4,3,2,2])};
        case_3_test = {InputType(vec![9]),       OutputType(vec![1,0])};
        case_4_test = {InputType(vec![9,9,9,9]), OutputType(vec![1,0,0,0,0])};
    )]
    fn plus_one_test(input: InputType, result: OutputType) -> () {
        assert_eq!(Solution::plus_one(input.0), result.0);
    }
}
