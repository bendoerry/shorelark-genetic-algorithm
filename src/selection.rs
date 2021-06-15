use crate::individual::Individual;

pub trait SelectionMethod {
    fn select<I>(&self, population: &[I]) -> &I
    where
        I: Individual;
}
