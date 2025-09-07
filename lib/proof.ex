defmodule CounterExample.Proof do
  alias Anoma.Arm.Resource
  alias Anoma.Arm.NullifierKey
  alias Anoma.Arm.MerklePath
  alias Anoma.Arm.MerkleTree
  alias Anoma.Arm.ComplianceUnit
  alias Anoma.Arm.ComplianceWitness
  alias Anoma.Arm
  alias CounterExample.CounterLogic
  alias CounterExample.CounterWitness
  alias CounterExample.NIF

  import Anoma.Util

  @doc """
  Generate a compliance proof for two resources.
  """
  @spec compliance(Resource.t(), NullifierKey.t(), MerklePath.t(), Resource.t()) ::
          {ComplianceUnit.t(), [byte()]}
  def compliance(consumed, consumed_nf, merkle_path, created) do
    compliance_witness =
      ComplianceWitness.from_resources_with_path(consumed, consumed_nf, merkle_path, created)

    compliance_unit = Arm.prove(compliance_witness)
    #
    {compliance_unit, compliance_witness.rcv}
  end

  @doc """
  Generate the logic proofs for the given resources.
  """
  @spec logic(Resource.t(), NullifierKey.t(), Resource.t(), [byte()]) ::
          {LogicProof.t(), LogicProof.t()}
  def logic(consumed, consumed_nf, created, discovery_pk) do
    nullifier = Resource.nullifier(consumed, consumed_nf)
    commitment = Resource.commitment(created)

    action_tree =
      MerkleTree.new([
        binlist2vec32(nullifier),
        binlist2vec32(commitment)
      ])

    # create the path of the nullifier and commitments in the action tree.
    consumed_resource_path = MerkleTree.path_of(action_tree, binlist2vec32(nullifier))
    created_resource_path = MerkleTree.path_of(action_tree, binlist2vec32(commitment))

    # counter logic for consumed resource
    {discovery_sk, _} = NIF.random_key_pair()

    consumed_counter_logic = %CounterLogic{
      witness: %CounterWitness{
        is_consumed: true,
        old_counter: consumed,
        old_counter_existence_path: consumed_resource_path,
        nf_key: consumed_nf,
        new_counter: created,
        new_counter_existence_path: created_resource_path,
        discovery_sk: discovery_sk,
        discovery_pk: discovery_pk,
        discovery_nonce: Anoma.Util.randombinlist(12)
      }
    }

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
