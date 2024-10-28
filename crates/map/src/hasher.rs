pub struct Hasher {
    seed: i32,
}

impl Hasher {
    pub fn new(seed: i32) -> Self {
        Self { seed }
    }

    pub fn hash(&self, input: &(f64, f64)) -> f64 {
        let m1 = (3.1251, 17.8737);
        let m2 = 43758.54321;
        let dot_val = input.0 * m1.0 + input.1 * m1.1;
        let sin_val = dot_val.sin();
        (sin_val * m2).fract()
    }
}

#[cfg(test)]
mod tests {
    use crate::hasher::Hasher;

    #[test]
    pub fn test() {
        let hasher = Hasher::new(123);
        let val = hasher.hash(&(1., 0.));
        let val2 = hasher.hash(&(1., 2.));
        let val3 = hasher.hash(&(2., 3.));
        let val4 = hasher.hash(&(0., 64.));
        let val5 = hasher.hash(&(64., 64.));
        println!("{:?}", (val, val2, val3, val4, val5));
    }
}
