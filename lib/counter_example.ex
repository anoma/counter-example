defmodule CounterExample do
  @moduledoc """
  Documentation for `CounterExample`.
  """

  alias Anoma.Arm
  alias Anoma.Arm.Action
  alias Anoma.Arm.DeltaWitness
  alias Anoma.Arm.MerklePath
  alias Anoma.Arm.NullifierKey
  alias Anoma.Arm.Transaction
  alias CounterExample.Create
  alias CounterExample.Proof

  def test do
    {key1, commitment1} = NullifierKey.random_pair()
    ephemeral = Create.create_ephemeral_counter(commitment1)

    {_key2, commitment2} = NullifierKey.random_pair()
    created = Create.create_new_counter(commitment2, ephemeral, key1)

    {compliance_unit, rcv} = Proof.compliance(ephemeral, key1, MerklePath.default(), created)

    # create a random keypair to encrypt the ciphertext
    sender_keypair = Arm.random_key_pair()
    receiver_keypair = Arm.random_key_pair()

    {consumed_proof, created_proof} =
      Proof.logic(ephemeral, key1, created, sender_keypair, receiver_keypair)

    consumed_proof = Arm.convert(consumed_proof)
    created_proof = Arm.convert(created_proof)

    # create an action for this transaction
    action = %Action{
      compliance_units: [compliance_unit],
      logic_verifier_inputs: [consumed_proof, created_proof]
    }

    delta_witness = %DeltaWitness{signing_key: rcv}

    transaction = %Transaction{
      actions: [action],
      delta_proof: {:witness, delta_witness},
      expected_balance: nil
    }

    # generate the delta proof for the transaction
    transaction = Transaction.generate_delta_proof(transaction)

    # verify the transaction
    if Arm.verify_transaction(transaction) do
      {:ok, transaction}
    else
      {:error, :verify_failed, transaction}
    end
  end
end
