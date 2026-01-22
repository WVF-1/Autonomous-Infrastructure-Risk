use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct SimulationRng {
    rng: StdRng,
}

impl SimulationRng {
    pub fn new(seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_entropy(),
        };
        
        Self { rng }
    }

    pub fn gen_bool(&mut self, p: f64) -> bool {
        self.rng.gen_bool(p)
    }

    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: rand::distributions::uniform::SampleUniform,
        R: rand::distributions::uniform::SampleRange<T>,
    {
        self.rng.gen_range(range)
    }
}