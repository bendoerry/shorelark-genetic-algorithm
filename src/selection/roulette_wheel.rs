use rand::seq::SliceRandom;

use crate::individual::Individual;
use crate::selection::SelectionMethod;

#[derive(Default)]
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

        // there is nothing special about this thousand; a number as low
        // as fifty might do the trick, too          v--v
        let actual_histogram: BTreeMap<i32, _> = (0..1000)
            .map(|_| method.select(&mut rng, &population))
            .fold(Default::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as _).or_default() += 1;

                histogram
            });

        let expected_histogram = maplit::btreemap! {
            // fitness => how many times this fitness has been chosen
            1 => 98,
            2 => 202,
            3 => 278,
            4 => 422,
        };

        assert_eq!(actual_histogram, expected_histogram);
    }
}
