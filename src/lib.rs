use individual::Individual;
use selection::SelectionMethod;

mod individual;
mod selection;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
    }

    pub fn evolve<I>(&self, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // TODO selection
                // TODO crossover
                // TODO mutation
                todo!()
            })
            .collect()
    }
}
