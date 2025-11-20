# Tasks (Rust)
**Практика в Rust через решение алгоритмических задач**

## Дополнительная информация
1. Все свспомогательные процессы (развёртывание, сборка, запуск и т.д.) автоматизированы при помощи [Fuzz](./Fuzz/README.md)
0. Для удобства проверки решений реализована поддержка параметрического тестирования:  
    пример для [two-sum](https://leetcode.com/problems/two-sum):  
    ```rs
    // leet_code/src/easy/two_sum.rs
    #[cfg(test)]
    mod tests {
        use super::*;

        extern crate leet_code_macro;
        use leet_code_macro::leet_code_test;

        #[leet_code_test(
            struct InputType(Vec<i32>, i32);
            struct OutputType(Vec<i32>);
            case_1_test = {InputType(vec![2, 7, 11, 15], 9), OutputType(vec![0, 1])};
            case_2_test = {InputType(vec![3,2,4], 6),        OutputType(vec![1, 2])};
            case_3_test = {InputType(vec![3,3], 6),          OutputType(vec![0, 1])};
        )]
        fn two_sum_test(input: InputType, result: OutputType) -> () {
            assert_eq!(Solution::two_sum(input.0, input.1), result.0);
        }
    }
    ```
    пример для [rotate-list](https://leetcode.com/problems/rotate-list):  
    ```rs
    // leet_code/src/medium/rotate_list.rs
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
    ```