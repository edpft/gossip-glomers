use std::iter;

#[derive(Debug)]
pub struct GrowOnlyCounter {
    counts: Vec<u32>,
}

impl GrowOnlyCounter {
    pub fn new(node_count: usize) -> Self {
        let counts = vec![0; node_count];
        Self { counts }
    }

    pub fn add_to_count(&mut self, index: usize, delta: u32) -> Option<()> {
        let maybe_current = self.counts.get_mut(index);
        let Some(current) = maybe_current else {
            return None;
        };
        *current += delta;
        Some(())
    }

    pub fn counts(&self) -> &Vec<u32> {
        self.counts.as_ref()
    }

    pub fn sum(&self) -> u32 {
        let sum: u32 = self.counts.iter().sum();
        sum
    }

    pub fn update_counts(&mut self, other_counts: &Vec<u32>) {
        let max = iter::zip(self.counts(), other_counts)
            .map(|(a, b)| a.max(b))
            .copied()
            .collect();
        self.counts = max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_new_is_zero() {
        let grow_only_counter = GrowOnlyCounter::new(5);
        assert_eq!(grow_only_counter.sum(), 0)
    }

    #[test]
    fn test_sum_after_add() {
        let mut grow_only_counter = GrowOnlyCounter::new(5);
        grow_only_counter
            .add_to_count(0, 1)
            .expect("Index 0 exists");
        grow_only_counter
            .add_to_count(0, 2)
            .expect("Index 0 exists");
        assert_eq!(grow_only_counter.sum(), 3)
    }

    #[test]
    fn test_compare_after_add() {
        let mut grow_only_counter = GrowOnlyCounter::new(5);
        grow_only_counter
            .add_to_count(0, 1)
            .expect("Index 0 exists");
        grow_only_counter
            .add_to_count(0, 2)
            .expect("Index 0 exists");
        assert_eq!(grow_only_counter.sum(), 3);

        let mut other_grow_only_counter = GrowOnlyCounter::new(5);
        other_grow_only_counter
            .add_to_count(1, 3)
            .expect("Index 1 exists");
        other_grow_only_counter
            .add_to_count(1, 4)
            .expect("Index 1 exists");
        assert_eq!(other_grow_only_counter.sum(), 7);

        grow_only_counter.update_counts(other_grow_only_counter.counts());
        assert_eq!(grow_only_counter.sum(), 10);
        assert_eq!(other_grow_only_counter.sum(), 7);

        grow_only_counter
            .add_to_count(0, 5)
            .expect("Index 0 exists");
        other_grow_only_counter
            .add_to_count(1, 6)
            .expect("Index 0 exists");
        assert_eq!(grow_only_counter.sum(), 15);
        assert_eq!(other_grow_only_counter.sum(), 13);
    }
}
