use rand::Rng;

use crate::chromosome::Chromosome;
use crate::crossover::CrossoverMethod;

#[derive(Clone, Debug)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn rand::RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        let parent_a = parent_a.iter();
        let parent_b = parent_b.iter();

        parent_a
            .zip(parent_b)
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::crossover::CrossoverMethod;

    use super::UniformCrossover;

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a = todo!();
        let parent_b = todo!();

        let child = UniformCrossover::new().crossover(&mut rng, &parent_a, &parent_b);

        assert!(/* ... */);
    }
}
