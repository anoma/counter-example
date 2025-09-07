#![cfg(feature = "nif")]

use crate::CounterWitness;
use arm::encryption::{AffinePoint, SecretKey};
use arm::merkle_path::MerklePath;
use arm::rustler_util::{at_struct, RustlerDecoder, RustlerEncoder};
use rustler::types::map::map_new;
use rustler::{atoms, Decoder, Encoder, Env, Error, NifResult, Term};

atoms! {
    at_counter_witness = "CounterExample.CounterWitness",
    at_is_consumed = "is_consumed",
    at_old_counter = "old_counter",
    at_old_counter_existence_path = "old_counter_existence_path",
    at_nf_key = "nf_key",
    at_new_counter = "new_counter",
    at_new_counter_existence_path = "new_counter_existence_path",
    at_discovery_pk = "discovery_pk",
    at_discovery_sk = "discovery_sk",
    at_discovery_nonce = "discovery_nonce",
}

impl RustlerEncoder for CounterWitness {
    fn rustler_encode<'a>(&self, env: Env<'a>) -> Result<Term<'a>, Error> {
        let map = map_new(env)
            .map_put(at_struct().encode(env), at_counter_witness().encode(env))?
            .map_put(at_is_consumed().encode(env), self.is_consumed.encode(env))?
            .map_put(at_old_counter().encode(env), self.old_counter.encode(env))?
            .map_put(
                at_old_counter_existence_path().encode(env),
                self.old_counter_existence_path.rustler_encode(env).unwrap(),
            )?
            .map_put(at_nf_key().encode(env), self.nf_key.encode(env))?
            .map_put(
                at_new_counter().encode(env),
                self.new_counter.rustler_encode(env).unwrap(),
            )?
            .map_put(
                at_new_counter_existence_path().encode(env),
                self.new_counter_existence_path.rustler_encode(env).unwrap(),
            )?
            .map_put(
                at_discovery_pk().encode(env),
                self.discovery_pk.rustler_encode(env).unwrap(),
            )?
            .map_put(
                at_discovery_sk().encode(env),
                self.discovery_sk.rustler_encode(env).unwrap(),
            )?
            .map_put(
                at_discovery_nonce().encode(env),
                self.discovery_nonce.to_vec().rustler_encode(env).unwrap(),
            )?;

        Ok(map)
    }
}

impl<'a> RustlerDecoder<'a> for CounterWitness {
    fn rustler_decode(term: Term<'a>) -> NifResult<Self> {
        let is_consumed_term = term.map_get(at_is_consumed().encode(term.get_env()))?;
        let is_consumed = is_consumed_term.decode()?;
        let old_counter_term = term.map_get(at_old_counter().encode(term.get_env()))?;
        let old_counter = old_counter_term.decode()?;
        let old_counter_existence_path_term =
        term.map_get(at_old_counter_existence_path().encode(term.get_env()))?;
        let old_counter_existence_path: MerklePath =
        RustlerDecoder::rustler_decode(old_counter_existence_path_term)?;
        let nf_key_term = term.map_get(at_nf_key().encode(term.get_env()))?;
        let nf_key = nf_key_term.decode()?;
        let new_counter_term = term.map_get(at_new_counter().encode(term.get_env()))?;
        let new_counter = new_counter_term.decode()?;
        let new_counter_existence_path_term =
        term.map_get(at_new_counter_existence_path().encode(term.get_env()))?;
        let new_counter_existence_path: MerklePath =
        RustlerDecoder::rustler_decode(new_counter_existence_path_term)?;
        let discovery_pk_term = term.map_get(at_discovery_pk().encode(term.get_env()))?;
        let discovery_pk: AffinePoint = RustlerDecoder::rustler_decode(discovery_pk_term)?;
        let discovery_sk_term = term.map_get(at_discovery_sk().encode(term.get_env()))?;
        let discovery_sk: SecretKey = discovery_sk_term.decode()?;
        let discovery_nonce_term = term.map_get(at_discovery_nonce().encode(term.get_env()))?;
        let discovery_nonce_vec: Vec<u8> = RustlerDecoder::rustler_decode(discovery_nonce_term)?;
        let discovery_nonce: [u8; 12] = discovery_nonce_vec.try_into().unwrap();

        let  counter_witness : CounterWitness = CounterWitness {
            is_consumed,
            old_counter,
            old_counter_existence_path,
            nf_key,
            new_counter,
            new_counter_existence_path,
            discovery_pk,
            discovery_sk,
            discovery_nonce,
        };
        println!("{:?}", counter_witness.clone());
        Ok(counter_witness)
    }
}

impl Encoder for CounterWitness {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let encoded = self.rustler_encode(env);
        encoded.expect("failed to encode CounterWitness")
    }
}

impl<'a> Decoder<'a> for CounterWitness {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        CounterWitness::rustler_decode(term)
    }
}
