#![feature(min_type_alias_impl_trait)]

use crossover::CrossoverMethod;
use individual::Individual;
use selection::SelectionMethod;

mod chromosome;
mod crossover;
mod individual;
mod selection;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S, crossover_method: impl CrossoverMethod + 'static) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
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

                // TODO crossover
                // TODO mutation
                // TODO convert `Chromosome` back into `Individual`
                todo!()
            })
            .collect()
    }
}
