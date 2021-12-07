pub trait MinMax: Iterator {
    fn min_max(self) -> Option<(Self::Item, Self::Item)>;
}

impl<I: Iterator<Item = T>, T: Ord + Copy> MinMax for I {
    fn min_max(mut self) -> Option<(Self::Item, Self::Item)> {
        let mut min = self.next()?;
        let mut max = min;

        for value in self {
            if value < min {
                min = value;
            } else if value > max {
                max = value;
            }
        }

        Some((min, max))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_works() {
        let nums = vec![5,1,4,2,3];
        assert_eq!(Some((1, 5)), nums.into_iter().min_max());
    }
}