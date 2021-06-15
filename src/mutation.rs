use crate::chromosome::Chromosome;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn rand::RngCore, child: &mut Chromosome);
}
