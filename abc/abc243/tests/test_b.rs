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
4
1 3 5 2
2 3 1 4
";
        let expected = "\
1
2
";
        run(input, expected);
    }

    #[test]
    fn sample2() {
        let input = "\
3
1 2 3
4 5 6
";
        let expected = "\
0
0
";
        run(input, expected);
    }

    #[test]
    fn sample3() {
        let input = "\
7
4 8 1 7 9 5 6
3 5 1 7 8 2 6
";
        let expected = "\
3
2
";
        run(input, expected);
    }
}
