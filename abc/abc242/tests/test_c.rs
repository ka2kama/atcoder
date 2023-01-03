mod tests {
    use cli_test_dir::*;
    use pretty_assertions::assert_eq;

    const BIN: &'static str = "./c";

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
2
";
        let expected = "\
25
";
        run(input, expected);
    }

    #[test]
    fn sample2() {
        let input = "\
4
";
        let expected = "\
203
";
        run(input, expected);
    }

    #[test]
    fn sample3() {
        let input = "\
1000000
";
        let expected = "\
248860093
";
        run(input, expected);
    }
}