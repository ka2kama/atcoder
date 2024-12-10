mod tests {
    use cli_test_dir::*;
    use pretty_assertions::assert_eq;

    const BIN: &'static str = "./b";

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
8
0 3 2 6 2 1 0 0
";
        let expected = "\
4
";
        run(input, expected);
    }

    #[test]
    fn sample2() {
        let input = "\
3
2000 2000 2000
";
        let expected = "\
0
";
        run(input, expected);
    }
}
