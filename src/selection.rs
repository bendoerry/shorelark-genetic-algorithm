use crate::individual::Individual;
pub use roulette_wheel::RouletteWheelSelection;

mod roulette_wheel;

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn rand::RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}
