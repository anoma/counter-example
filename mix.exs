defmodule CounterExample.MixProject do
  use Mix.Project

  def project do
    [
      app: :counter_example,
      version: "0.1.0",
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:anoma_sdk, git: "git@github.com:anoma/anoma-sdk.git", branch: "main"},
      {:rustler, "~> 0.36.1", runtime: false},
      {:typed_struct, "~> 0.3.0"}
    ]
  end
end
