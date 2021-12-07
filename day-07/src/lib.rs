use std::ops::Sub;
use crate::minmax::MinMax;

mod minmax;

pub fn optimal_position(numbers: Vec<usize>) -> Option<(usize, usize)> {
    let (min, max) = numbers.iter().min_max()?;

    (*min..=*max)
        .map(|position| (position, fuel_consumption(&numbers, position)))
        .min_by_key(|(_, fuel)| *fuel)
}

fn fuel_consumption<'a>(numbers: impl IntoIterator<Item = &'a usize>, x: usize) -> usize {
    numbers.into_iter().map(|n| n.fuel_to(x)).sum()
}

trait FuelTo {
    fn fuel_to(self, target: Self) -> Self;
}

impl FuelTo for usize {
    fn fuel_to(self, target: Self) -> Self {
        self.diff(target)
    }
}

pub trait Diff: Ord + Sub + Sized {
    /// Returns the absolute difference between `self` and `b`
    ///
    /// #Examples
    ///
    /// ```
    /// use day_07::Diff;
    ///
    /// assert_eq!(3.diff(1), 2);
    /// ```
    fn diff(self, b: Self) -> Self::Output;
}

impl<T: Ord + Sub + Sized> Diff for T {
    fn diff(self, b: Self) -> Self::Output {
        if self >= b {
            self - b
        } else {
            b - self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimal_position_works() {
        let positions = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(Some((2, 37)), optimal_position(positions));
    }

    #[test]
    fn fuel_consumption_works() {
        let positions = vec![1,2,4,5];
        assert_eq!(fuel_consumption(&positions, 3), 6);
        assert_eq!(fuel_consumption(&positions, 1), 8);
    }
}