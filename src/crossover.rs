use crate::chromosome::Chromosome;
pub use uniform::UniformCrossover;

mod uniform;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn rand::RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}
