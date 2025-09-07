defmodule CounterExample do
  @moduledoc """
  Documentation for `CounterExample`.
  """

  alias CounterExample.NIF
  alias CounterExample.Create
  alias CounterExample.Proof
  alias Anoma.Arm
  alias Anoma.Arm.Transaction
  alias Anoma.Arm.MerklePath
  alias Anoma.Arm.DeltaWitness
  alias Anoma.Arm.Action
  alias Anoma.Util

  def test do
    {_discovery_sk, discovery_pk} = NIF.random_key_pair()
    {key1, commitment1} = Anoma.Arm.NullifierKey.random_pair()
    ephemeral = Create.create_ephemeral_counter(commitment1)

    {_key2, commitment2} = Anoma.Arm.NullifierKey.random_pair()
    created = Create.create_new_counter(commitment2, ephemeral, key1)

    {compliance_unit, rcv} = Proof.compliance(ephemeral, key1, MerklePath.default(), created)

    {consumed_proof, created_proof} = Proof.logic(ephemeral, key1, created, discovery_pk)

    consumed_proof = Arm.convert(consumed_proof)
    created_proof = Arm.convert(created_proof)

    # create an action for this transaction
    action = %Action{
      compliance_units: [compliance_unit],
      logic_verifier_inputs: [consumed_proof, created_proof]
    }

    delta_witness = %DeltaWitness{signing_key: Util.binlist2bin(rcv)}

    transaction = %Transaction{
      actions: [action],
      delta_proof: {:witness, delta_witness}
    }

    # generate the delta proof for the transaction
    transaction = Transaction.generate_delta_proof(transaction)
    transaction
  end
end
