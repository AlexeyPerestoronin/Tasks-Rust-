extern crate proc_macro;
use proc_macro::TokenStream;
use regex::Regex;

/// A procedural macro attribute for generating automated test cases for LeetCode problems.
///
/// This macro simplifies the process of writing unit tests for LeetCode solutions by automatically
/// generating individual `#[test]` functions for each test case specified in the macro's attributes.
/// It parses the provided input and output type definitions, along with test case data, and creates
/// separate test functions that call the annotated function with the corresponding test data.
///
/// # Macro Attribute Format
///
/// The macro expects its attributes to be in the following format:
///
/// ```rust,ignore
/// #[leet_code_test(
///     struct InputType(/* fields */);
///     struct OutputType(/* fields */);
///     test_case_name1 = {InputType(/* input data */), OutputType(/* expected output */)};
///     test_case_name2 = {InputType(/* input data */), OutputType(/* expected output */)};
///     // ... more test cases
/// )]
/// ```
///
/// - `InputType` and `OutputType` are struct definitions that represent the input parameters and
///   expected output for the test function.
/// - Each test case is defined as `name = {InputType(...), OutputType(...)}`, where the name
///   becomes the generated test function name.
///
/// # Generated Code
///
/// For each test case, the macro generates a `#[test]` function that:
/// 1. Defines the `InputType` and `OutputType` structs locally.
/// 2. Creates a `TestCaseData` struct containing the input and expected output.
/// 3. Calls the original annotated function with the input and expected output as arguments.
///
/// The annotated function should accept two parameters: one of type `InputType` and one of type
/// `OutputType`, typically performing assertions to verify the solution's correctness.
///
/// # Example Usage
///
/// ```rust,ignore
/// use leet_code_macro::leet_code_test;
///
/// #[leet_code_test(
///     struct InputType(Vec<i32>, i32);
///     struct OutputType(Vec<i32>);
///     example1 = {InputType(vec![2,7,11,15], 9), OutputType(vec![0,1])};
///     example2 = {InputType(vec![3,2,4], 6), OutputType(vec![1,2])};
/// )]
/// fn test_two_sum(input: InputType, expected: OutputType) {
///     let (nums, target) = input.0;
///     let result = two_sum(nums, target);
///     assert_eq!(result, expected.0);
/// }
/// ```
///
/// This generates two test functions: `example1` and `example2`, each calling `test_two_sum`
/// with the respective test data.
///
/// # Implementation Details
///
/// The macro uses regular expressions to parse the attribute string and extract:
/// - The test function name from the annotated item.
/// - The `InputType` and `OutputType` struct definitions.
/// - Individual test case names and their associated data.
///
/// It then constructs the generated code as a string and parses it back into a `TokenStream`.
///
/// # Dependencies
///
/// This macro relies on the `regex` crate for parsing the attribute strings.
///
#[proc_macro_attribute]
pub fn leet_code_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = attr.to_string();
    let test_case = item.to_string();

    let test_case_function_name = Regex::new(r"fn ([\w+_]+)")
        .expect("cannot create regexp for capture the name of test case function")
        .captures(&test_case)
        .expect("cannot capture the name of test case function")
        .get(1)
        .expect("cannot obtain the name of test case function");
    let test_case_function_name = test_case_function_name.as_str();

    let input_type = Regex::new(r"(struct\s+InputType\(.+?\));")
        .expect("cannot create regexp for InputType")
        .captures(&attr)
        .expect("cannot capture InputType in macro arguments")
        .get(1)
        .expect("cannot obtain InputType from capture");
    let input_type = input_type.as_str();

    let output_type = Regex::new(r"(struct\s+OutputType\(.+?\));")
        .expect("cannot create regexp for OutputType")
        .captures(&attr)
        .expect("cannot capture OutputType in macro arguments")
        .get(1)
        .expect("cannot obtain OutputType from capture");
    let output_type = output_type.as_str();

    let mut result = String::new();

    let test_cases = Regex::new(r"([\w_]+)\s+=\s+\{(InputType\(.+?\), OutputType\(.+?\))\};")
        .expect("cannot create regexp for test cases");

    for (_, [case_name, case_data]) in test_cases
        .captures_iter(&attr)
        .map(|test_case| test_case.extract())
    {
        result.push_str(
            format!(
                "
#[test]
fn {case_name}() -> () {{
    #[derive(Debug)]
    {input_type};
    #[derive(Debug)]
    {output_type};
    struct TestCaseData(InputType, OutputType);
    let case_data = TestCaseData({case_data});
    {test_case}
    {test_case_function_name}(case_data.0, case_data.1);
}}
        "
            )
            .as_str(),
        );
    }

    // println!("result: {result}");
    result.parse().unwrap()
}
