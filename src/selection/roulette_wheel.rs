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
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::individual::TestIndividual;
    use crate::selection::roulette_wheel::RouletteWheelSelection;
    use crate::selection::SelectionMethod;

    #[test]
    fn test_select() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let actual = RouletteWheelSelection::new().select(&mut rng, &population);

        assert!(/* what here? */)
    }
}
