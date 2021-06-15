use crate::individual::Individual;

pub trait SelectionMethod {
    fn select<'a, I>(&self, population: &'a [I]) -> &'a I
    where
        I: Individual;
}
