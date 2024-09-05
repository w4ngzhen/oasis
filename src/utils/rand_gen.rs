use rand::{Rng, RngCore, SeedableRng};
use rand_pcg::{Lcg128Xsl64, Pcg64};
use std::ops::Range;

pub struct RandGen {
    rng: Lcg128Xsl64,
}

impl RandGen {
    pub fn new(seed: Option<i32>) -> RandGen {
        let rng_seed = if let Some(seed) = seed { seed } else { 0 };
        let rng = Pcg64::seed_from_u64(rng_seed as u64);
        RandGen { rng }
    }

    #[allow(unused)]
    pub fn next_i32(&mut self) -> i32 {
        self.rng.next_u32() as i32
    }

    pub fn range(&mut self, from: i32, to: i32) -> i32 {
        let range: Range<i32> = if from <= to { from..to } else { to..from };
        self.rng.gen_range(range)
    }

    pub fn roll_dice(&mut self, dice_count: i32, dice_size: i32) -> i32 {
        let mut result = 0_i32;
        for _ in 0..dice_count {
            result += self.rng.gen_range(1..dice_size + 1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::rand_gen::RandGen;

    #[test]
    fn test_range() {
        let mut rand_gen = RandGen::new(None);
        let result = rand_gen.range(0, 99);
        println!("result = {}", result);
    }

    #[test]
    fn test_dice() {
        let mut rand_gen = RandGen::new(None);
        let result = rand_gen.roll_dice(8, 6);
        println!("result = {}", result);
    }
}
