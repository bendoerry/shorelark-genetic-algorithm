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

        let mut child = Vec::new();
        let gene_count = parent_a.len();

        for gene_idx in 0..gene_count {
            let gene = if rng.gen_bool(0.5) {
                parent_a[gene_idx]
            } else {
                parent_b[gene_idx]
            };

            child.push(gene);
        }

        child.into_iter().collect()
    }
}