use rand::Rng;

use crate::chromosome::Chromosome;
use crate::mutation::MutationMethod;

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    /// Probability of changing a gene:
    /// - 0.0 = no genes will be touched
    /// - 1.0 = all genes will be touched
    chance: f32,

    /// Magnitude of that change:
    /// - 0.0 = touched genes will not be modified
    /// - 3.0 = touched genes will be += or -= by at most 3.0
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn rand::RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>()
            }
        }
    }
}

#[cfg(test)]
mod tests {

    mod given_zero_chance {
        mod and_zero_coefficient {

            #[test]
            fn does_not_change_the_original_chromosome() {
                todo!();
            }
        }

        mod and_nonzero_coefficient {

            #[test]
            fn does_not_change_the_original_chromosome() {
                todo!();
            }
        }
    }

    mod given_fifty_fifty_chance {

        mod and_zero_coefficient {

            #[test]
            fn does_not_change_the_original_chromosome() {
                todo!();
            }
        }

        mod and_nonzero_coefficient {

            #[test]
            fn slightly_changes_the_original_chromosome() {
                todo!();
            }
        }
    }

    mod given_max_chance {

        mod and_zero_coefficient {

            #[test]
            fn does_not_change_the_original_chromosome() {
                todo!();
            }
        }

        mod and_nonzero_coefficient {

            #[test]
            fn entirely_changes_the_original_chromosome() {
                todo!();
            }
        }
    }
}
