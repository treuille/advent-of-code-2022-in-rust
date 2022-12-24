pub mod parse_grid {
    use ndarray::{Array1, Array2};

    /// Parses a sring representing a grid of characters into a 2D array.
    pub fn parse_char_grid<T, F>(input: &str, parse_char: F) -> Array2<T>
    where
        F: Fn(char) -> T,
    {
        let lines: Vec<&str> = input.trim().lines().collect();
        let rows = lines.len();
        let input = lines
            .iter()
            .flat_map(|line| line.trim().chars())
            .map(parse_char)
            .collect::<Array1<T>>();
        let cols = input.len() / rows;
        input.into_shape((rows, cols)).unwrap()
    }
}

pub mod parse_regex {
    use regex::Regex;
    use std::marker::PhantomData;
    use std::str::Lines;

    pub fn parse_line<'a, T>(re: &Regex, s: &'a str) -> T
    where
        T: FromRegex<'a>,
    {
        T::parse(re, s)
    }

    pub fn parse_lines<'a, T>(re: Regex, s: &'a str) -> ParseLines<'a, T>
    where
        T: FromRegex<'a>,
    {
        ParseLines {
            re,
            lines: s.trim().lines(),
            _phantom: PhantomData,
        }
    }

    pub trait FromStr<'a> {
        fn from_str(s: &'a str) -> Self;
    }

    impl<'a> FromStr<'a> for &'a str {
        fn from_str(s: &'a str) -> Self {
            s
        }
    }

    impl FromStr<'_> for i64 {
        fn from_str(s: &'_ str) -> Self {
            s.parse().unwrap()
        }
    }

    impl FromStr<'_> for u64 {
        fn from_str(s: &'_ str) -> Self {
            s.parse().unwrap()
        }
    }

    impl FromStr<'_> for isize {
        fn from_str(s: &'_ str) -> Self {
            s.parse().unwrap()
        }
    }

    impl FromStr<'_> for usize {
        fn from_str(s: &'_ str) -> Self {
            s.parse().unwrap()
        }
    }

    impl FromStr<'_> for char {
        fn from_str(s: &'_ str) -> Self {
            s.parse().unwrap()
        }
    }

    pub trait FromRegex<'a> {
        fn parse(re: &Regex, s: &'a str) -> Self;
    }

    impl<'a, T1> FromRegex<'a> for (T1,)
    where
        T1: FromStr<'a>,
    {
        fn parse(re: &Regex, s: &'a str) -> Self {
            let captures = re
                .captures(s)
                .unwrap_or_else(|| panic!("Can't parse: \"{s}\""));
            (T1::from_str(captures.get(1).unwrap().as_str()),)
        }
    }

    impl<'a, T1, T2> FromRegex<'a> for (T1, T2)
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
    {
        fn parse(re: &Regex, s: &'a str) -> Self {
            let captures = re
                .captures(s)
                .unwrap_or_else(|| panic!("Can't parse: \"{s}\""));
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
            )
        }
    }

    impl<'a, T1, T2, T3> FromRegex<'a> for (T1, T2, T3)
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
        T3: FromStr<'a>,
    {
        fn parse(re: &Regex, s: &'a str) -> Self {
            let captures = re
                .captures(s)
                .unwrap_or_else(|| panic!("Can't parse: \"{s}\""));
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
                T3::from_str(captures.get(3).unwrap().as_str()),
            )
        }
    }

    impl<'a, T1, T2, T3, T4> FromRegex<'a> for (T1, T2, T3, T4)
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
        T3: FromStr<'a>,
        T4: FromStr<'a>,
    {
        fn parse(re: &Regex, s: &'a str) -> Self {
            let captures = re
                .captures(s)
                .unwrap_or_else(|| panic!("Can't parse: \"{s}\""));
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
                T3::from_str(captures.get(3).unwrap().as_str()),
                T4::from_str(captures.get(4).unwrap().as_str()),
            )
        }
    }

    impl<'a, T1, T2, T3, T4, T5> FromRegex<'a> for (T1, T2, T3, T4, T5)
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
        T3: FromStr<'a>,
        T4: FromStr<'a>,
        T5: FromStr<'a>,
    {
        fn parse(re: &Regex, s: &'a str) -> Self {
            let captures = re
                .captures(s)
                .unwrap_or_else(|| panic!("Can't parse: \"{s}\""));
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
                T3::from_str(captures.get(3).unwrap().as_str()),
                T4::from_str(captures.get(4).unwrap().as_str()),
                T5::from_str(captures.get(5).unwrap().as_str()),
            )
        }
    }

    impl<'a, T1, T2, T3, T4, T5, T6> FromRegex<'a> for (T1, T2, T3, T4, T5, T6)
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
        T3: FromStr<'a>,
        T4: FromStr<'a>,
        T5: FromStr<'a>,
        T6: FromStr<'a>,
    {
        fn parse(re: &Regex, s: &'a str) -> Self {
            let captures = re
                .captures(s)
                .unwrap_or_else(|| panic!("Can't parse: \"{s}\""));
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
                T3::from_str(captures.get(3).unwrap().as_str()),
                T4::from_str(captures.get(4).unwrap().as_str()),
                T5::from_str(captures.get(5).unwrap().as_str()),
                T6::from_str(captures.get(6).unwrap().as_str()),
            )
        }
    }

    impl<'a, T1, T2, T3, T4, T5, T6, T7> FromRegex<'a> for (T1, T2, T3, T4, T5, T6, T7)
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
        T3: FromStr<'a>,
        T4: FromStr<'a>,
        T5: FromStr<'a>,
        T6: FromStr<'a>,
        T7: FromStr<'a>,
    {
        fn parse(re: &Regex, s: &'a str) -> Self {
            let captures = re
                .captures(s)
                .unwrap_or_else(|| panic!("Can't parse: \"{s}\""));
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
                T3::from_str(captures.get(3).unwrap().as_str()),
                T4::from_str(captures.get(4).unwrap().as_str()),
                T5::from_str(captures.get(5).unwrap().as_str()),
                T6::from_str(captures.get(6).unwrap().as_str()),
                T7::from_str(captures.get(7).unwrap().as_str()),
            )
        }
    }

    pub struct ParseLines<'a, T>
    where
        T: FromRegex<'a>,
    {
        re: Regex,
        lines: Lines<'a>,
        _phantom: PhantomData<T>,
    }

    impl<'a, T> Iterator for ParseLines<'a, T>
    where
        T: FromRegex<'a>,
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            self.lines.next().map(|line| T::parse(&self.re, line))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_grid::parse_char_grid;
    use super::parse_regex::{parse_line, parse_lines};
    use ndarray::{arr2, Array2};
    use regex::Regex;

    #[test]
    fn pair_parse_regex() {
        let re = Regex::new(r"(\d+) -> ([a-z]+)").unwrap();
        let (a, b): (usize, &str) = parse_line(&re, "1 -> abc");
        assert_eq!(a, 1);
        assert_eq!(b, "abc");
    }

    #[test]
    fn triple_parse_regex() {
        let re = Regex::new(r"(\d+) \+\+ ([a-z]+) \+\+ ([a-z])").unwrap();
        let (a, b, c): (usize, &str, char) = parse_line(&re, "99 ++ xyz ++ q");
        assert_eq!(a, 99);
        assert_eq!(b, "xyz");
        assert_eq!(c, 'q');
    }

    #[test]
    fn quadruple_parse_regex() {
        let re = Regex::new(r"(\-?\d+),(\d+),(\d+),(\d+)").unwrap();
        let (a, b, c, d): (isize, usize, usize, usize) = parse_line(&re, "-1,0,1,2");
        assert_eq!(a, -1);
        assert_eq!(b, 0);
        assert_eq!(c, 1);
        assert_eq!(d, 2);
    }

    #[test]
    fn lines_parse_regexs() {
        let input = "
        a => x 
        b => y 
        c => z 
        ";

        let re = Regex::new(r"([a-z]) => ([a-z])").unwrap();
        let mut iter = parse_lines(re, input);

        assert_eq!(Some(('a', 'x')), iter.next());
        assert_eq!(Some(('b', 'y')), iter.next());
        assert_eq!(Some(('c', 'z')), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_grid_parsing() {
        let to_u32 = |c: char| c.to_digit(10).unwrap() as u32;
        let input = "\n123\n 456\n";
        let array = parse_char_grid(input, to_u32);
        let expected = arr2(&[[1, 2, 3], [4, 5, 6]]);
        assert_eq!(array.dim(), (2, 3));
        assert_eq!(array, expected);
    }
}
