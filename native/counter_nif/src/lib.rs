use arm::logic_proof::{LogicProver, LogicVerifier};
use counter_library::CounterLogic;
use rustler::nif;

#[nif]
pub fn counter_verifying_key_nif() -> Vec<u8> {
    CounterLogic::verifying_key_as_bytes()
}

#[nif]
pub fn prove_counter_logic(counter_logic: CounterLogic) -> LogicVerifier {
    let logic_verifier = counter_logic.prove();
    logic_verifier
}
rustler::init!("Elixir.CounterExample.NIF");
