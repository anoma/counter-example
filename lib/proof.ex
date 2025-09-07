defmodule CounterExample.Proof do
  @moduledoc """
  This module defines functions to generate proofs for the counter logic and the
  compliance.
  """
  alias Anoma.Arm
  alias Anoma.Arm.ComplianceUnit
  alias Anoma.Arm.ComplianceWitness
  alias Anoma.Arm.MerklePath
  alias Anoma.Arm.MerkleTree
  alias Anoma.Arm.LogicVerifier
  alias Anoma.Arm.NullifierKey
  alias Anoma.Arm.Resource
  alias CounterExample.CounterLogic
  alias CounterExample.CounterWitness
  alias CounterExample.NIF
  alias Anoma.Arm.Keypair

  @doc """
  Generate a compliance proof for two resources.
  """
  @spec compliance(Resource.t(), NullifierKey.t(), MerklePath.t(), Resource.t()) ::
          {ComplianceUnit.t(), binary()}
  def compliance(consumed, consumed_nf, merkle_path, created) do
    compliance_witness =
      ComplianceWitness.from_resources_with_path(consumed, consumed_nf, merkle_path, created)

    compliance_unit = Arm.prove_compliance_witness(compliance_witness)
    #
    {compliance_unit, compliance_witness.rcv}
  end

  @doc """
  Generate the logic proofs for the given resources.
  """
  @spec logic(Resource.t(), NullifierKey.t(), Resource.t(), Keypair.t(), Keypair.t()) ::
          {LogicVerifier.t(), LogicVerifier.t()}
  def logic(consumed, consumed_nf, created, sender, receiver) do
    nullifier = Resource.nullifier(consumed, consumed_nf)
    commitment = Resource.commitment(created)

    action_tree = MerkleTree.new([nullifier, commitment])

    # create the path of the nullifier and commitments in the action tree.
    consumed_resource_path = MerkleTree.path_of(action_tree, nullifier)
    created_resource_path = MerkleTree.path_of(action_tree, commitment)

    consumed_counter_logic = %CounterLogic{
      witness: %CounterWitness{
        is_consumed: true,
        old_counter: consumed,
        old_counter_existence_path: consumed_resource_path,
        nf_key: consumed_nf,
        new_counter: created,
        new_counter_existence_path: created_resource_path,
        discovery_sk: sender.secret_key,
        discovery_pk: receiver.public_key,
        discovery_nonce: :crypto.strong_rand_bytes(12)
      }
    }

    # something with the keys might be wrong

    # generate the proof for the consumed counter
    consumed_logic_proof = NIF.prove_counter_logic(consumed_counter_logic)

    # create a proof for the created counter
    created_counter_logic = %CounterLogic{
      witness: %{consumed_counter_logic.witness | is_consumed: false}
    }

    created_logic_proof = NIF.prove_counter_logic(created_counter_logic)

    {consumed_logic_proof, created_logic_proof}
  end
end
