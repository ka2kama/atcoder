mod tests {
    use cli_test_dir::*;
    use pretty_assertions::assert_eq;

    const BIN: &'static str = "./a";

    fn run(input: &str, expected: &str) {
        let output = TestDir::new(BIN, "")
            .cmd()
            .output_with_stdin(input)
            .expect_success();
        assert_eq!(output.stdout_str(), expected);
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample1() {
        let input = "\
1 2 1 1000
";
        let expected = "\
Yes
";
        run(input, expected);
    }

    #[test]
    fn sample2() {
        let input = "\
3 5
";
        let expected = "\
No
";
        run(input, expected);
    }

    #[test]
    fn sample3() {
        let input = "\
1 10
";
        let expected = "\
Yes
";
        run(input, expected);
    }
}
