use arm::logic_proof::LogicProver;
use arm::{
    encryption::AffinePoint, merkle_path::MerklePath, nullifier_key::NullifierKey,
    resource::Resource,
};
use counter_witness::CounterWitness;
use hex::FromHex;
use lazy_static::lazy_static;
use risc0_zkvm::Digest;
use serde::{Deserialize, Serialize};

pub const SIMPLE_COUNTER_ELF: &[u8] = include_bytes!("../elf/counter-guest.bin");
lazy_static! {
    pub static ref SIMPLE_COUNTER_ID: Digest =
        Digest::from_hex("f35d75340a5facf56be07082de0fc261e1ad0b6d80971fed5de8d6d935ae0a9f")
            .unwrap();
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct CounterLogic {
    witness: CounterWitness,
}

impl CounterLogic {
    pub fn new(
        is_consumed: bool,
        old_counter: Resource,
        old_counter_existence_path: MerklePath,
        nf_key: NullifierKey,
        new_counter: Resource,
        new_counter_existence_path: MerklePath,
        discovery_pk: AffinePoint,
    ) -> Self {
        Self {
            witness: CounterWitness::new(
                is_consumed,
                old_counter,
                old_counter_existence_path,
                nf_key,
                new_counter,
                new_counter_existence_path,
                discovery_pk,
            ),
        }
    }
}

impl LogicProver for CounterLogic {
    type Witness = CounterWitness;
    fn proving_key() -> &'static [u8] {
        SIMPLE_COUNTER_ELF
    }

    fn verifying_key() -> Digest {
        *SIMPLE_COUNTER_ID
    }

    fn witness(&self) -> &Self::Witness {
        &self.witness
    }
}
