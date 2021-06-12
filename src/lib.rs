pub mod test_support {

    pub struct TestCase<T> {
        pub input: &'static str,
        pub expected: T,
    }
}
