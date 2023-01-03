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
2 10
3 6
4 5
";
        let expected = "\
Yes
";
        run(input, expected);
    }

    #[test]
    fn sample2() {
        let input = "\
2 10
10 100
10 100
";
        let expected = "\
No
";
        run(input, expected);
    }

    #[test]
    fn sample3() {
        let input = "\
4 12
1 8
5 7
3 4
2 6
";
        let expected = "\
Yes
";
        run(input, expected);
    }
}
