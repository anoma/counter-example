use crate::init::{ephemeral_counter, init_counter_resource};
use crate::util::{generate_compliance_proof, generate_logic_proofs};
use arm::action::Action;
use arm::compliance::INITIAL_ROOT;
use arm::delta_proof::DeltaWitness;
use arm::encryption::{random_keypair, Ciphertext, SecretKey};
use arm::merkle_path::MerklePath;
use arm::nullifier_key::NullifierKey;
use arm::transaction::{Delta, Transaction};
use arm::utils::words_to_bytes;
use eth::submit;
use hex::ToHex;
use k256::elliptic_curve::group::GroupEncoding;
use k256::elliptic_curve::PrimeField;
use k256::{AffinePoint, CompressedPoint, Scalar};
use rand::random;
use runtime::Builder;
use tokio::runtime;

mod eth;
mod init;
mod util;

fn create_transaction(discovery_pk: AffinePoint, discovery_sk: SecretKey) -> Transaction {
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

fn print_keys(discovery_sk: SecretKey, discovery_pk: AffinePoint) -> (String, String) {
    // encode the secret key as a hex string and print it.
    let discovery_sk_str: String = discovery_sk
        .inner()
        .to_bytes()
        .as_slice()
        .to_vec()
        .encode_hex();
    println!(
        "discovery secret key: {} (len: {})",
        discovery_sk_str,
        discovery_sk_str.len()
    );

    // encode the private key as a hex string and print it.
    let discovery_pk_str: String = discovery_pk.to_bytes().as_slice().to_vec().encode_hex();
    println!(
        "discovery private key: {} (len: {})",
        discovery_pk_str,
        discovery_pk_str.len()
    );

    // return both strings
    (discovery_sk_str, discovery_pk_str)
}

fn recover_keys(discovery_sk_str: String, discovery_pk_str: String) -> (SecretKey, AffinePoint) {
    // decode the secret key from hex.
    let discovery_sk_regenerated_bytes: [u8; 32] =
        hex::decode(discovery_sk_str).unwrap().try_into().unwrap();
    let discovery_sk = SecretKey::new(
        // Scalar::from_repr(discovery_sk_regenerated_bytes.try_into().unwrap()).unwrap(),
        Scalar::from_repr(discovery_sk_regenerated_bytes.into()).unwrap(),
    );

    // decode the private key from hex.
    let discovery_pk_regenerated_bytes: Vec<u8> = hex::decode(discovery_pk_str).unwrap();
    let bytes: [u8; 33] = discovery_pk_regenerated_bytes.try_into().unwrap();
    let discovery_pk = AffinePoint::from_bytes(CompressedPoint::from_slice(&bytes)).unwrap();

    (discovery_sk, discovery_pk)
}

fn main() {
    let x = INITIAL_ROOT.as_words().to_vec();
    let y = words_to_bytes(x.as_slice());
    println!("x: {:?} {:?}", x, y);
    // let (discovery_sk, discovery_pk) = random_keypair();
    // let (discovery_sk_str, discovery_pk_str) = print_keys(discovery_sk.clone(), discovery_pk);
    // let (discovery_sk_rec, discovery_pk_rec) = recover_keys(discovery_sk_str, discovery_pk_str);
    // print_keys(discovery_sk_rec, discovery_pk_rec);
    //
    // let thing_to_encrypt: Vec<u8> = (0..66).map(|_| random::<u8>()).collect();
    //
    // let nonce_bytes: Vec<u8> = (0..12).map(|_| random::<u8>()).collect();
    // let nonce: [u8; 12] = nonce_bytes.try_into().unwrap();
    //
    // let cipher_text : Ciphertext = Ciphertext::encrypt(&thing_to_encrypt, &discovery_pk, &discovery_sk, nonce);
    //
    // let decipher_result = cipher_text.decrypt(&discovery_sk);
    // match decipher_result {
    //     Ok(_) => {
    //         println!("deciphered the discovery payload");
    //     }
    //     Err(_) => {
    //         println!("Decryption failed, exiting");
    //     }
    // }
    //
    //
    //
    // // let tx = create_transaction(discovery_pk, discovery_sk);
    // // submit_transaction(tx);

    println!("transaction generated");
}
