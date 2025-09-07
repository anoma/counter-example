use arm::action_tree::MerkleTree;
use arm::compliance::ComplianceWitness;
use arm::compliance_unit::ComplianceUnit;
use arm::encryption::AffinePoint;
use arm::logic_proof::{LogicProver, LogicVerifier};
use arm::merkle_path::MerklePath;
use arm::nullifier_key::NullifierKey;
use arm::resource::Resource;
use counter_library::CounterLogic;

pub fn convert_counter_to_value_ref(value: u128) -> Vec<u8> {
    let mut arr = [0u8; 32];
    let bytes = value.to_le_bytes();
    arr[..16].copy_from_slice(&bytes); // left-align, right-pad with 0
    arr.to_vec()
}

pub fn generate_compliance_proof(
    consumed_counter: Resource,
    nf_key: NullifierKey,
    merkle_path: MerklePath,
    created_counter: Resource,
) -> (ComplianceUnit, Vec<u8>) {
    let compliance_witness = ComplianceWitness::from_resources_with_path(
        consumed_counter,
        nf_key,
        merkle_path,
        created_counter,
    );
    let compliance_unit = ComplianceUnit::create(&compliance_witness);
    (compliance_unit, compliance_witness.rcv)
}

pub fn generate_logic_proofs(
    consumed_counter: Resource,
    nf_key: NullifierKey,
    consumed_discovery_pk: AffinePoint,
    created_counter: Resource,
    created_discovery_pk: AffinePoint,
) -> Vec<LogicVerifier> {
    let consumed_counter_nf = consumed_counter.nullifier(&nf_key).unwrap();
    let created_counter_cm = created_counter.commitment();

    let action_tree = MerkleTree::new(vec![consumed_counter_nf, created_counter_cm]);

    let consumed_counter_path = action_tree.generate_path(&consumed_counter_nf).unwrap();
    let created_counter_path = action_tree.generate_path(&created_counter_cm).unwrap();

    let consumed_counter_logic = CounterLogic::new(
        true,
        consumed_counter.clone(),
        consumed_counter_path.clone(),
        nf_key.clone(),
        created_counter.clone(),
        created_counter_path.clone(),
        consumed_discovery_pk,
    );
    let consumed_logic_proof = consumed_counter_logic.prove();

    let created_counter_logic = CounterLogic::new(
        false,
        consumed_counter,
        consumed_counter_path,
        nf_key,
        created_counter,
        created_counter_path,
        created_discovery_pk,
    );
    let created_logic_proof = created_counter_logic.prove();

    vec![consumed_logic_proof, created_logic_proof]
}
