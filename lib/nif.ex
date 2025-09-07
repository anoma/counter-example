defmodule CounterExample.NIF do
  @moduledoc """
  I define a few functions work with the counter application.
  """

  alias Anoma.Arm.LogicVerifier
  alias CounterExample.CounterLogic

  use Rustler,
    otp_app: :counter_example,
    crate: :counter_nif

  @doc """
  Returns the logic ref for the counter binary.
  """
  @spec counter_logic_ref :: binary()
  def counter_logic_ref do
    counter_verifying_key_nif()
    |> :binary.list_to_bin()
  end

  @doc """
  Prove a counter logic witness and return a logic proof.
  """
  @spec prove_counter_logic(CounterLogic.t()) :: LogicVerifier.t()
  def prove_counter_logic(_), do: :erlang.nif_error(:nif_not_loaded)

  # ----------------------------------------------------------------------------#
  #                                Helpers                                     #
  # ----------------------------------------------------------------------------#

  # NIF implementation.
  # Wrapped by counter_logic_ref/0 to return a binary rather than a list of bytes.
  @spec counter_verifying_key_nif :: [byte()]
  defp counter_verifying_key_nif, do: :erlang.nif_error(:nif_not_loaded)
end
