#![cfg(feature = "nif")]

use crate::CounterLogic;
use arm::rustler_util::{at_struct, RustlerDecoder, RustlerEncoder};
use counter_witness::CounterWitness;
use rustler::types::map::map_new;
use rustler::{atoms, Decoder, Encoder, Env, NifResult, Term};

atoms! {
    at_counter_logic = "Elixir.CounterExample.CounterLogic",
    at_witness = "witness"

}

impl Encoder for CounterLogic {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let map = map_new(env)
            .map_put(at_struct().encode(env), at_counter_logic().encode(env))
            .unwrap()
            .map_put(
                at_witness().encode(env),
                self.witness.rustler_encode(env).unwrap(),
            )
            .expect("failed");
        map
    }
}

impl<'a> Decoder<'a> for CounterLogic {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let witness_term = term.map_get(at_witness().encode(term.get_env()))?;
        let witness: CounterWitness = RustlerDecoder::rustler_decode(witness_term)?;
        Ok(CounterLogic { witness })
    }
}
