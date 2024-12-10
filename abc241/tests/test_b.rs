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
3 2
1 1 3
3 1
";
        let expected = "\
Yes
";
        run(input, expected);
    }

    #[test]
    fn sample2() {
        let input = "\
1 1
1000000000
1
";
        let expected = "\
No
";
        run(input, expected);
    }

    #[test]
    fn sample3() {
        let input = "\
5 2
1 2 3 4 5
5 5
";
        let expected = "\
No
";
        run(input, expected);
    }
}
