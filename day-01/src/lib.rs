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

pub struct Windows<T, I: Iterator<Item = T>, const WINDOW_SIZE: usize> {
    current_window: [Option<T>; WINDOW_SIZE],
    iter: I,
}

impl<T: Clone, I: Iterator<Item = T>, const WINDOW_SIZE: usize> Windows<T, I, WINDOW_SIZE> {
    const INIT: Option<T> = None;

    pub fn new(mut iter: I) -> Self {
        let mut current_window: [Option<T>; WINDOW_SIZE] = [Self::INIT; WINDOW_SIZE];

        for i in 0..WINDOW_SIZE - 1 {
            current_window[i] = iter.next();
        }

        Self {
            current_window,
            iter,
        }
    }

    fn prepare_return_value(&self) -> Option<[T; WINDOW_SIZE]> {
        let mut return_value: [Option<T>; WINDOW_SIZE] = [Self::INIT; WINDOW_SIZE];

        for i in 0..WINDOW_SIZE {
            if let Some(value) = &self.current_window[i] {
                return_value[i] = Some(value.clone());
            } else {
                return None;
            }
        }

        let return_value = return_value.map(Option::unwrap);
        Some(return_value)
    }
}

impl<T: Clone, I: Iterator<Item = T>, const WINDOW_SIZE: usize> Iterator for Windows<T, I, WINDOW_SIZE> {
    type Item = [T; WINDOW_SIZE];

    fn next(&mut self) -> Option<Self::Item> {
        self.current_window[WINDOW_SIZE - 1] = self.iter.next();

        let returned = self.prepare_return_value();

        for i in 0..WINDOW_SIZE - 1 {
            self.current_window[i] = self.current_window[i + 1].take();
        }

        returned
    }
}

impl<T: Debug, I: Iterator<Item = T>, const WINDOW_SIZE: usize> Debug for Windows<T, I, WINDOW_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Windows")
            .field("current_window", &self.current_window)
            .finish()
    }
}

pub trait WindowsExt<T>: Iterator<Item = T> {
    fn windows<const WINDOW_SIZE: usize>(self) -> Windows<T, Self, WINDOW_SIZE>
    where
        Self: Sized;
}

impl<T: Clone, I: Iterator<Item = T>> WindowsExt<T> for I {
    fn windows<const WINDOW_SIZE: usize>(self) -> Windows<T, Self, WINDOW_SIZE> {
        Windows::new(self)
    }
}

pub fn three_measurement_sums(
    measurements: impl Iterator<Item = u32>,
) -> impl Iterator<Item = u32> {
    measurements.windows::<3>().map(|window| {
        window
            .into_iter()
            .sum()
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
        let mut windows = vec.into_iter().windows::<3>();

        assert_eq!(windows.next(), Some([1, 2, 3]));
        assert_eq!(windows.next(), Some([2, 3, 4]));
        assert_eq!(windows.next(), Some([3, 4, 5]));
        assert_eq!(windows.next(), None);
    }
}
