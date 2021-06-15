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
    fn test_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a = (1..=100).map(|n| n as f32).collect();
        let parent_b = (1..=100).map(|n| -n as f32).collect();

        // First parent will be:
        //   [1, 2, /* ... */, 100]
        //
        // Second parent will look similar, but with reversed signs:
        //   [-1, -2, /* ... */, -100]
        //
        // Just like in the histogram, the concrete number of genes doesn't
        // matter - 100 will nicely round up to 100%, that's all

        let child = UniformCrossover::new().crossover(&mut rng, &parent_a, &parent_b);

        // Number of genes different between `child` and `parent_a`
        let diff_a = child.iter().zip(parent_a).filter(|(c, p)| *c != p).count();

        // Number of genes different between `child` and `parent_b`
        let diff_b = child.iter().zip(parent_b).filter(|(c, p)| *c != p).count();

        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }
}
