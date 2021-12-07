use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input() -> impl Iterator<Item = u32> {
    File::open("./input.txt")
        .map(|file| BufReader::new(file))
        .unwrap()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<u32>().unwrap())
}

pub fn count_increments(mut measurements: impl Iterator<Item = u32>) -> u32 {
    let mut increments = 0_u32;
    let mut current = if let Some(first) = measurements.next() {
        first
    } else {
        return 0;
    };

    while let Some(next) = measurements.next() {
        if next > current {
            increments += 1;
        }

        current = next;
    }

    increments
}

pub struct Windows<T, I: Iterator<Item = T>> {
    window_size: usize,
    current_window: Vec<Option<T>>,
    iter: I,
}

impl<T, I: Iterator<Item = T>> Windows<T, I> {
    pub fn new(mut iter: I, window_size: usize) -> Self {
        let mut current_window = Vec::with_capacity(window_size);

        for _ in 0..window_size - 1 {
            current_window.push(iter.next());
        }

        current_window.push(None);

        Self {
            window_size,
            current_window,
            iter,
        }
    }
}

impl<T: Clone, I: Iterator<Item = T>> Iterator for Windows<T, I> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_window[self.window_size - 1] = self.iter.next();

        let returned: Option<Vec<T>> = self
            .current_window
            .iter()
            .cloned()
            .collect();

        for i in 0..self.window_size - 1 {
            self.current_window[i] = self.current_window[i + 1].take();
        }

        returned
    }
}

impl<T: Debug, I: Iterator<Item = T>> Debug for Windows<T, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Windows")
            .field("current_window", &self.current_window)
            .finish()
    }
}

pub trait WindowsExt<T>: Iterator<Item = T> {
    fn windows(self, window_size: usize) -> Windows<T, Self>
    where
        Self: Sized;
}

impl<T, I: Iterator<Item = T>> WindowsExt<T> for I {
    fn windows(self, window_size: usize) -> Windows<T, Self> {
        Windows::new(self, window_size)
    }
}

pub fn three_measurement_sums(
    measurements: impl Iterator<Item = u32>,
) -> impl Iterator<Item = u32> {
    measurements.windows(3).map(|window| {
        window
            .into_iter()
            .fold(0_u32, |current, next| current + next)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increments_works() {
        let measurements: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increments(measurements.into_iter()), 7);
    }

    #[test]
    fn three_measurement_sums_works() {
        let measurements = vec![1_u32, 2, 3, 4, 5].into_iter();
        let mut iter = three_measurement_sums(measurements);

        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn puzzle_2_works() {
        let measurements = vec![199_u32, 200, 208, 210, 200, 207, 240, 269, 260, 263].into_iter();
        let measurements = three_measurement_sums(measurements);
        let increments = count_increments(measurements);

        assert_eq!(increments, 5);
    }

    #[test]
    fn windows_works() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut windows = vec.into_iter().windows(3);

        assert_eq!(windows.next(), Some(vec![1, 2, 3]));
        assert_eq!(windows.next(), Some(vec![2, 3, 4]));
        assert_eq!(windows.next(), Some(vec![3, 4, 5]));
        assert_eq!(windows.next(), None);
    }
}
