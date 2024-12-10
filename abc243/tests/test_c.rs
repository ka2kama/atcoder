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
3
2 3
1 1
4 1
RRL
";
        let expected = "\
Yes
";
        run(input, expected);
    }

    #[test]
    fn sample2() {
        let input = "\
2
1 1
2 1
RR
";
        let expected = "\
No
";
        run(input, expected);
    }

    #[test]
    fn sample3() {
        let input = "\
10
1 3
1 4
0 0
0 2
0 4
3 1
2 4
4 2
4 4
3 3
RLRRRLRLRR
";
        let expected = "\
Yes
";
        run(input, expected);
    }
}
