use crate::chromosome::Chromosome;
pub use gaussian::GaussianMutation;

mod gaussian;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn rand::RngCore, child: &mut Chromosome);
}
