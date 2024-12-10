mod tests {
    use cli_test_dir::*;
    use pretty_assertions::assert_eq;

    const BIN: &'static str = "./d";

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
6
1 4 1 2 2 1
";
        let expected = "\
3
";
        run(input, expected);
    }

    #[test]
    fn sample2() {
        let input = "\
1
1
";
        let expected = "\
1
";
        run(input, expected);
    }

    #[test]
    fn sample3() {
        let input = "\
11
3 1 4 1 5 9 2 6 5 3 5
";
        let expected = "\
7
";
        run(input, expected);
    }
}
