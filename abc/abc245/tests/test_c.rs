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
5 4
9 8 3 7 2
1 6 2 9 5
";
        let expected = "\
Yes
";
        run(input, expected);
    }

    #[test]
    fn sample2() {
        let input = "\
4 90
1 1 1 100
1 2 3 100
";
        let expected = "\
No
";
        run(input, expected);
    }

    #[test]
    fn sample3() {
        let input = "\
4 1000000000
1 1 1000000000 1000000000
1 1000000000 1 1000000000
";
        let expected = "\
Yes
";
        run(input, expected);
    }
}
