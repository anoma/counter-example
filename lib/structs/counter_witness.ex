defmodule CounterExample.CounterWitness do
  @moduledoc """
  I define the datastructure `CounterWitness` that defines the structure of a
  counter logic witness for the resource machine.
  """
  use TypedStruct

  alias Anoma.Arm.MerklePath
  alias Anoma.Arm.NullifierKey
  alias Anoma.Arm.Resource

  typedstruct do
    field(:is_consumed, boolean())
    field(:old_counter, Resource.t())
    field(:old_counter_existence_path, MerklePath.t())
    field(:nf_key, NullifierKey.t())
    field(:new_counter, Resource.t())
    field(:new_counter_existence_path, MerklePath.t())
    field(:discovery_pk, binary())
    field(:discovery_sk, binary())
    field(:discovery_nonce, binary())
  end
end
