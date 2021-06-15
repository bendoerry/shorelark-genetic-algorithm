use rand::seq::SliceRandom;

use crate::individual::Individual;
use crate::selection::SelectionMethod;

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn rand::RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::individual::{Individual, TestIndividual};
    use crate::selection::roulette_wheel::RouletteWheelSelection;
    use crate::selection::SelectionMethod;

    #[test]
    fn test_select() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let method = RouletteWheelSelection::new();

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut actual_histogram = BTreeMap::new();

        //               there is nothing special about this thousand;
        //          v--v a number as low as fifty might do the trick, too
        for _ in 0..1000 {
            let fitness = method.select(&mut rng, &population).fitness() as i32;

            *actual_histogram.entry(fitness).or_insert(0) += 1;
        }

        let expected_histogram = BTreeMap::from_iter(vec![
            // (fitness, how many times this fitness has been chosen)
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
        ]);

        assert_eq!(actual_histogram, expected_histogram);
    }
}
