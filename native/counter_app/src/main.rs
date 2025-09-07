use crate::init::{ephemeral_counter, init_counter_resource};
use crate::util::{generate_compliance_proof, generate_logic_proofs};
use arm::action::Action;
use arm::delta_proof::DeltaWitness;
use arm::encryption::{random_keypair, Ciphertext};
use arm::merkle_path::MerklePath;
use arm::nullifier_key::NullifierKey;
use arm::transaction::{Delta, Transaction};
use eth::submit;
use runtime::Builder;
use tokio::runtime;

mod eth;
mod init;
mod util;

fn create_transaction() -> Transaction {
    let (discovery_sk, discovery_pk) = random_keypair();

    // create a keypair to create the ephemeral counter
    let (eph_nf_key, eph_nf_key_cm) = NullifierKey::random_pair();

    // create an ephemeral counter
    let ephemeral_counter = ephemeral_counter(eph_nf_key_cm);

    // create a keypair to create the counter
    let (_nf_key, nf_key_cm) = NullifierKey::random_pair();

    // create the counter resource
    let created_counter = init_counter_resource(&ephemeral_counter, &eph_nf_key, &nf_key_cm);

    // generate the compliance proof for the resources
    let (compliance_unit, rcv) = generate_compliance_proof(
        ephemeral_counter.clone(),
        eph_nf_key.clone(),
        MerklePath::default(),
        created_counter.clone(),
    );

    // generate the logic verifier inputs
    let logic_verifier_inputs = generate_logic_proofs(
        ephemeral_counter.clone(),
        eph_nf_key,
        discovery_pk,
        created_counter.clone(),
        discovery_pk,
    );

    // create the actions for the transaction
    let action = Action::new(vec![compliance_unit], logic_verifier_inputs);

    // create the transaction delta proof
    let delta_witness = DeltaWitness::from_bytes(&rcv);
    let mut tx = Transaction::create(vec![action], Delta::Witness(delta_witness));
    tx.generate_delta_proof();

    // given the discovery private key, check if the ciphertext can be deciphered

    let discovery_ciphertext = Ciphertext::from_words(
        &tx.actions[0].logic_verifier_inputs[0]
            .app_data
            .discovery_payload[0]
            .blob,
    );
    // check if its decipherable
    let decipher_result = discovery_ciphertext.decrypt(&discovery_sk);
    match decipher_result {
        Ok(_) => {
            println!("deciphered the discovery payload");
        }
        Err(_) => {
            println!("Decryption failed, exiting");
        }
    }

    // verify the transaction
    if tx.clone().verify() {
        println!("Transaction verified");
    } else {
        println!("Transaction not verified");
    }
    tx
}

pub fn submit_transaction(transaction: Transaction) {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let _ = rt.block_on(async { submit(transaction).await });
}

fn main() {
    let tx = create_transaction();
    let _ = submit_transaction(tx);
    println!("yeet");
}
