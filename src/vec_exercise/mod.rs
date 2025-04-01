#[derive(Debug)]
struct Splitter<'a> {
    input: &'a str,
    delimiter: char,
    start: usize,
}

impl<'a> Splitter<'a> {
    fn new(input: &'a str, delimiter: char) -> Self {
        Splitter {
            input,
            delimiter,
            start: 0,
        }
    }
}

impl<'a> Iterator for Splitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.input.len() {
            return None;
        }

        if self.start == self.input.len() {
            if self.input.is_empty() {
                self.start += 1;
                return Some("");
            }

            if self.input.chars().last().unwrap() == self.delimiter {
                self.start += 1;
                return Some("");
            }
            return None;
        }

        let end = self.input[self.start..].find(self.delimiter);

        match end {
            Some(index) => {
                let result = &self.input[self.start..self.start + index];
                self.start += index + 1;
                Some(result)
            }
            None => {
                let result = &self.input[self.start..];
                self.start = self.input.len();
                Some(result)
            }
        }
    }
}

pub fn run_impl_iterator_exercise_for_collect_method() {
    let test_case = vec![
        ("Multiple", "ab,cd,ef", ","),
        ("Multiple", "ab,cd,ef,", ","),
        ("Multiple", "ab,cd,ef", ":"),
        ("Multiple", "", ","),
        ("Multiple", "asasd-3333-asasdsda-d", "-"),
    ];

    for (name, input, delimiter) in test_case {
        let delimiter_char = delimiter.chars().next().unwrap();
        let expected: Vec<_> = input.split(delimiter_char).collect::<Vec<_>>();
        let splitter = Splitter::new(input, delimiter_char).collect::<Vec<_>>();

        println!("Test: {}", name);
        println!("Expected: {:?}", expected);
        println!("Splitter: {:?}", splitter);
        assert_eq!(expected, splitter);
    }
}
