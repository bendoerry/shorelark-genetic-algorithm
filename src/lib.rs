#![feature(min_type_alias_impl_trait)]

pub use crossover::{CrossoverMethod, UniformCrossover};
pub use individual::Individual;
pub use mutation::{GaussianMutation, MutationMethod};
pub use selection::{RouletteWheelSelection, SelectionMethod};

mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn rand::RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();

                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::crossover::UniformCrossover;
    use crate::individual::{Individual, TestIndividual};
    use crate::mutation::GaussianMutation;
    use crate::selection::RouletteWheelSelection;

    use super::GeneticAlgorithm;

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();

        TestIndividual::create(chromosome)
    }

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
        ];

        // We're running `.evolve()` a few times, so that the
        // differences between initial and output population are
        // easier to spot.
        //
        // No particular reason for a number of 10 - this test would
        // be fine for 5, 20 or even 1000 generations; the only thing
        // that'd change is the *magnitude* of difference between
        // initial and output population.
        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population);
        }

        let expected_population = vec![
            individual(&[0.44769490, 2.0648358, 4.3058133]), // fitness ~= 6.8
            individual(&[1.21268670, 1.5538777, 2.8869110]), // fitness ~= 5.7
            individual(&[1.06176780, 2.2657390, 4.4287640]), // fitness ~= 7.8
            individual(&[0.95909685, 2.4618788, 4.0247330]), // fitness ~= 7.4
        ];

        assert_eq!(population, expected_population);
    }
}
