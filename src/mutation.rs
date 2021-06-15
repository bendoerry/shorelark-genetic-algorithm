use crate::chromosome::Chromosome;

mod gaussian;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn rand::RngCore, child: &mut Chromosome);
}
