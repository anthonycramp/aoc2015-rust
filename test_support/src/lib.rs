pub mod test_support {

    pub struct TestCase<T> {
        pub input: &'static str,
        pub expected: T,
    }

    pub fn run_tests<T: std::fmt::Debug + std::cmp::PartialEq>(
        fn_under_test: fn(input: &str) -> T,
        test_cases: &[TestCase<T>],
    ) {
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(fn_under_test(input), *expected);
        }
    }
}
