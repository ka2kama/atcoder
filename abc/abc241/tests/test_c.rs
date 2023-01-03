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
8
........
........
.#.##.#.
........
........
........
........
........
";
        let expected = "\
Yes
";
        run(input, expected);
    }

    #[test]
    fn sample2() {
        let input = "\
6
######
######
######
######
######
######
";
        let expected = "\
Yes
";
        run(input, expected);
    }

    #[test]
    fn sample3() {
        let input = "\
10
..........
#..##.....
..........
..........
....#.....
....#.....
.#...#..#.
..........
..........
..........
";
        let expected = "\
No
";
        run(input, expected);
    }

    #[test]
    fn sample4() {
        let input = "\
10
..........
#..##.....
..........
..........
....#.....
....#.....
.#...#..#.
......#...
.......#..
..........
";
        let expected = "\
Yes
";
        run(input, expected);
    }

    #[test]
    fn sample5() {
        let input = "\
10
..........
#..##.....
..........
....#.....
....#.....
....#.....
.#...#..#.
..........
..........
..........
";
        let expected = "\
Yes
";
        run(input, expected);
    }
}
