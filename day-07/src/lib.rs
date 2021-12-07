use std::ops::Sub;
use crate::minmax::MinMax;

mod minmax;

pub fn optimal_position<F: FuelCalculator>(numbers: Vec<usize>) -> Option<(usize, usize)> {
    let (min, max) = numbers.iter().min_max()?;

    (*min..=*max)
        .map(|position| (position, fuel_consumption::<F, _>(&numbers, position)))
        .min_by_key(|(_, fuel)| *fuel)
}

fn fuel_consumption<'a, F: FuelCalculator, I: IntoIterator<Item = &'a usize>>(numbers: I, x: usize) -> usize {
    numbers.into_iter().map(|n| F::fuel(*n, x)).sum()
}

pub trait FuelCalculator {
    fn fuel(position: usize, target: usize) -> usize;
}

pub struct Part1;

impl FuelCalculator for Part1 {
    fn fuel(position: usize, target: usize) -> usize {
        position.diff(target)
    }
}

pub struct Part2ManualSum;

impl FuelCalculator for Part2ManualSum {
    fn fuel(position: usize, target: usize) -> usize {
        (1..=position.diff(target)).sum()
    }
}

pub struct Part2Triangular;

impl FuelCalculator for Part2Triangular {
    fn fuel(position: usize, target: usize) -> usize {
        let diff = position.diff(target) as f64;
        let fuel = (diff.powi(2) + diff) / 2.0;
        fuel as usize
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
    fn optimal_position_part_1() {
        let positions = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(Some((2, 37)), optimal_position::<Part1>(positions));
    }

    #[test]
    fn optimal_position_part_2() {
        let positions = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(Some((5, 168)), optimal_position::<Part2Triangular>(positions));
    }

    #[test]
    fn fuel_consumption_part_1() {
        let positions = vec![1,2,4,5];
        assert_eq!(fuel_consumption::<Part1, _>(&positions, 3), 6);
        assert_eq!(fuel_consumption::<Part1, _>(&positions, 1), 8);
    }

    #[test]
    fn fuel_consumption_part_2_manual_sum() {
        let positions = vec![1,2,4,5];
        assert_eq!(fuel_consumption::<Part2ManualSum, _>(&positions, 3), 8);
        assert_eq!(fuel_consumption::<Part2ManualSum, _>(&positions, 1), 17);
    }

    #[test]
    fn fuel_consumption_part_2_triangular() {
        let positions = vec![1,2,4,5];
        assert_eq!(fuel_consumption::<Part2Triangular, _>(&positions, 3), 8);
        assert_eq!(fuel_consumption::<Part2Triangular, _>(&positions, 1), 17);
    }
}