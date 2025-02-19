use rand_core::RngCore;
use std::ops::Range;

/// Generate a random bool
pub fn random_bool(rng: &mut dyn RngCore) -> bool {
    rng.next_u64() % 2 == 0
}

/// Shuffle a slice in place
pub fn shuffle<T>(slice: &mut [T], rng: &mut dyn RngCore) {
    let len = slice.len();
    for i in (1..len).rev() {
        let j = (rng.next_u64() as usize) % (i + 1);
        slice.swap(i, j);
    }
}

/// Generate a random number in the range [min, max)
pub fn random_range(range: Range<usize>, rng: &mut dyn RngCore) -> usize {
    assert!(range.start < range.end, "start must be less than end");
    let d = range.end - range.start;
    let random_value = (rng.next_u64() as usize) % d;
    range.start + random_value
}

/// Generate a vector of random indexes
pub fn generate_random_order_indexes(size: usize, rng: &mut dyn RngCore) -> Vec<usize> {
    let mut indexes: Vec<usize> = (0..size).collect();
    shuffle(&mut indexes, rng);
    indexes
}

pub fn generate_eq_clocked_times(
    size: usize,
    eq_clocked_times: usize,
    rng: &mut dyn RngCore,
) -> Vec<usize> {
    (0..size)
        .map(|_| random_range(0..eq_clocked_times, rng))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::SeedableRng;
    use rand_pcg::Lcg128Xsl64;

    #[test]
    fn test_random_order_indexes() {
        let mut rng1 = Lcg128Xsl64::seed_from_u64(5);
        let indexes1 = generate_random_order_indexes(10, &mut rng1);
        let indexes2 = generate_random_order_indexes(10, &mut rng1);
        assert!(indexes1 != indexes2);
        let mut rng2 = Lcg128Xsl64::seed_from_u64(5);
        let indexes3 = generate_random_order_indexes(10, &mut rng2);
        assert!(indexes1 == indexes3);
    }

    #[test]
    fn test_generate_eq_clocked_times() {
        let mut rng1 = Lcg128Xsl64::seed_from_u64(5);
        let indexes1 = generate_eq_clocked_times(10, 3, &mut rng1);
        let indexes2 = generate_eq_clocked_times(10, 3, &mut rng1);
        assert!(indexes1 != indexes2);
        let mut rng2 = Lcg128Xsl64::seed_from_u64(5);
        let indexes3 = generate_eq_clocked_times(10, 3, &mut rng2);
        assert!(indexes1 == indexes3);
    }
}
