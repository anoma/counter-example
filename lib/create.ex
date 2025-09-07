defmodule CounterExample.Create do
  @moduledoc """
  Logic to create counter resources.
  """

  alias Anoma.Arm.NullifierKeyCommitment
  alias Anoma.Arm.NullifierKey
  alias Anoma.Arm.Resource
  alias CounterExample.NIF
  alias Anoma.Util

  @doc """
  Create a new ephemeral counter.

  A counter is represented by its owner, and a unique label.
  """
  @spec create_ephemeral_counter(NullifierKeyCommitment.t()) :: Resource.t()
  def create_ephemeral_counter(commitment) do
    # the counter value is little endian encoded, padded to 32 bytes.
    counter_value =
      0
      |> :binary.encode_unsigned(:little)
      |> Util.pad_bitstring(32)
      |> Util.bin2binlist()

    # Create a counter resource
    %Resource{
      logic_ref: NIF.counter_logic_ref(),
      label_ref: Util.randombinlist(32),
      quantity: 1,
      value_ref: counter_value,
      is_ephemeral: true,
      nonce: Util.randombinlist(32),
      nk_commitment: commitment
    }
  end

  @doc """
  Given an ephemeral counter, creates a new counter to be created.

  The ephemeral counter serves as the resource we are consuming, in order to
  creeate the new counter.
  """
  @spec create_new_counter(NullifierKeyCommitment.t(), Resource.t(), NullifierKey.t()) ::
          Resource.t()
  def create_new_counter(commitment, ephemeral_counter, ephemeral_counter_nf_key) do
    IO.inspect(binding())
    # the counter value is little endian encoded, padded to 32 bytes.
    counter_value =
      1
      |> :binary.encode_unsigned(:little)
      |> Util.pad_bitstring(32)
      |> Util.bin2binlist()
      |> Enum.reverse()

    %{
      ephemeral_counter
      | is_ephemeral: false,
        rand_seed: Util.randombinlist(32),
        nonce: Resource.nullifier(ephemeral_counter, ephemeral_counter_nf_key),
        value_ref: counter_value,
        nk_commitment: commitment
    }
  end
end
