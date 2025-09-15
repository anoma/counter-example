defmodule CounterExample.CounterLogic do
  @moduledoc """
  I define the datastructure `CounterLogic` that defines the structure of a
  trivial logic witness for the resource machine.
  """
  use TypedStruct

  alias CounterExample.CounterWitness

  typedstruct do
    field(:witness, CounterWitness.t())
  end
end
