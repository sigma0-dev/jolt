use ark_ff::PrimeField;
use rand::prelude::StdRng;

use super::JoltInstruction;
use crate::{
    jolt::subtable::{eq::EqSubtable, LassoSubtable},
    utils::instruction_utils::chunk_and_concatenate_operands,
};

#[derive(Copy, Clone, Default, Debug)]
pub struct BNEInstruction(pub u64, pub u64);

impl JoltInstruction for BNEInstruction {
    fn operands(&self) -> [u64; 2] {
        [self.0, self.1]
    }

    fn combine_lookups<F: PrimeField>(&self, vals: &[F], _: usize, _: usize) -> F {
        F::one() - vals.iter().product::<F>()
    }

    fn g_poly_degree(&self, C: usize) -> usize {
        C
    }

    fn subtables<F: PrimeField>(&self, _: usize) -> Vec<Box<dyn LassoSubtable<F>>> {
        vec![Box::new(EqSubtable::new())]
    }

    fn to_indices(&self, C: usize, log_M: usize) -> Vec<usize> {
        chunk_and_concatenate_operands(self.0, self.1, C, log_M)
    }

    fn lookup_entry_u64(&self) -> u64 {
        (self.0 != self.1).into()
    }

    fn random(&self, rng: &mut StdRng) -> Self {
        use rand_core::RngCore;
        Self(rng.next_u32() as u64, rng.next_u32() as u64)
    }
}

#[cfg(test)]
mod test {
    use ark_curve25519::Fr;
    use ark_std::{test_rng, Zero};
    use rand_chacha::rand_core::RngCore;

    use crate::{jolt::instruction::JoltInstruction, jolt_instruction_test};

    use super::BNEInstruction;

    #[test]
    fn bne_instruction_32_e2e() {
        let mut rng = test_rng();
        const C: usize = 4;
        const M: usize = 1 << 16;

        for _ in 0..256 {
            let (x, y) = (rng.next_u32() as u64, rng.next_u32() as u64);
            let instruction = BNEInstruction(x, y);
            let expected = instruction.lookup_entry_u64();
            jolt_instruction_test!(instruction, expected.into());
            assert_eq!(
                instruction.lookup_entry::<Fr>(C, M),
                expected.into()
            );
        }
        for _ in 0..256 {
            let x = rng.next_u32() as u64;
            let instruction = BNEInstruction(x, x);
            jolt_instruction_test!(instruction, Fr::zero());
            assert_eq!(instruction.lookup_entry::<Fr>(C, M), Fr::zero());
        }
    }

    #[test]
    fn bne_instruction_64_e2e() {
        let mut rng = test_rng();
        const C: usize = 8;
        const M: usize = 1 << 16;

        for _ in 0..256 {
            let (x, y) = (rng.next_u64(), rng.next_u64());
            let instruction = BNEInstruction(x, y);
            let expected = instruction.lookup_entry_u64();
            jolt_instruction_test!(instruction, expected.into());
            assert_eq!(
                instruction.lookup_entry::<Fr>(C, M),
                expected.into()
            );
        }
        for _ in 0..256 {
            let x = rng.next_u64();
            let instruction = BNEInstruction(x, x);
            jolt_instruction_test!(instruction, Fr::zero());
            assert_eq!(instruction.lookup_entry::<Fr>(C, M), Fr::zero());
        }
    }

    use crate::jolt::instruction::test::{lookup_entry_u64_parity_random, lookup_entry_u64_parity};

    #[test]
    fn u64_parity() {
        let concrete_instruction = BNEInstruction(0, 0);
        lookup_entry_u64_parity_random::<Fr, BNEInstruction>(100, concrete_instruction);

        // Test edge-cases
        let u32_max: u64 = u32::MAX as u64;
        let instructions = vec![
            BNEInstruction(100, 0),
            BNEInstruction(0, 100),
            BNEInstruction(1 , 0),
            BNEInstruction(0, u32_max),
            BNEInstruction(u32_max, 0),
            BNEInstruction(u32_max, u32_max),
            BNEInstruction(u32_max, 1 << 8),
            BNEInstruction(1 << 8, u32_max),
        ];
        lookup_entry_u64_parity::<Fr, _>(instructions);
    }
}
