use crate::util::convert_counter_to_value_ref;
use arm::logic_proof::LogicProver;
use arm::nullifier_key::{NullifierKey, NullifierKeyCommitment};
use arm::resource::Resource;
use counter_library::CounterLogic;
use rand::Rng;

// It creates a random label reference and a nullifier key for the
// ephermeral counter resource.
pub fn ephemeral_counter(nf_key_cm: NullifierKeyCommitment) -> Resource {
    let mut rng = rand::thread_rng();
    let label_ref: [u8; 32] = rng.gen(); // Random label reference, it should be unique for each counter
    let nonce: [u8; 32] = rng.gen(); // Random nonce for the ephemeral resource
    Resource::create(
        CounterLogic::verifying_key_as_bytes(),
        label_ref.to_vec(),
        1,
        convert_counter_to_value_ref(0u128), // Initialize with value/counter 0
        true,
        nonce.to_vec(),
        nf_key_cm,
    )
}

// This function initializes a counter resource from an ephemeral counter
// resource and its nullifier key. It sets the resource as non-ephemeral, renews
// its randomness, resets the nonce from the ephemeral counter, and sets the
// value reference to 1 (the initial counter value). It also renews the
// nullifier key(commitment) for the counter resource.
pub fn init_counter_resource(
    ephemeral_counter: &Resource,
    ephemeral_counter_nf_key: &NullifierKey,
    nf_key_cm: &NullifierKeyCommitment,
) -> Resource {
    let mut init_counter = ephemeral_counter.clone();
    init_counter.is_ephemeral = false;
    init_counter.reset_randomness();
    init_counter.set_nonce_from_nf(ephemeral_counter, ephemeral_counter_nf_key);
    init_counter.set_value_ref(convert_counter_to_value_ref(1u128));
    init_counter.set_nf_commitment(nf_key_cm.clone());
    init_counter
}
