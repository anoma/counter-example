defmodule CounterExampleTest do
  use ExUnit.Case
  doctest CounterExample

  test "create ephemeral counter" do
    counter = CounterExample.NIF.create_ephemeral_counter()
  end
end
