defmodule CounterExample.NIF do
  @moduledoc """
  I define a few functions work with the counter application.
  """

  alias Anoma.Examples.Counter.CounterLogic
  alias Anoma.Arm.LogicVerifier

  use Rustler,
    otp_app: :counter_example,
    crate: :counter_nif

  @doc """
  Generates a random private key (Scalar) and its corresponding public key (ProjectivePoint)
  """
  @spec random_key_pair :: {binary(), [byte()]}
  def random_key_pair, do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Returns the logic ref for the counter binary.
  """
  @spec counter_logic_ref :: [byte()]
  def counter_logic_ref, do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Prove a counter logic witness and return a logic proof.
  """
  @spec prove_counter_logic(CounterLogic.t()) :: LogicVerifier.t()
  def prove_counter_logic(_), do: :erlang.nif_error(:nif_not_loaded)
end
