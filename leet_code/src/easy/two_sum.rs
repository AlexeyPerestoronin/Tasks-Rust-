extern crate leet_code_macro;
use leet_code_macro::make_answer;

make_answer!();
pub struct Solution;

impl Solution {
    pub fn two_sum(_nums: Vec<i32>, _target: i32) -> Vec<i32> {
        return Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        println!("{}", answer());
        Solution::two_sum(Vec::new(), 3);
    }
}
