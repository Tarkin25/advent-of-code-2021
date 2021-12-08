#[macro_export]
macro_rules! lines {
    () => {
        {
            use ::std::fs::File;
            use ::std::io::{BufRead, BufReader};

            File::open("input.txt")
            .map(BufReader::new)
            .unwrap()
            .lines()
            .map(|line| line.unwrap())
        }
    };
}

#[macro_export]
macro_rules! comma_separated {
    ($T:ty) => {
        ::std::fs::read_to_string("input.txt")
        .unwrap()
        .split(",")
        .map(<$T as ::std::str::FromStr>::from_str)
        .map(|r| r.unwrap())
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
