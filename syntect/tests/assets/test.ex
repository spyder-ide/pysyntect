defmodule Module do
  use GenServer

  @const [2, 3, 4]

  defmacro this_macro do
    quote do
      unquote(:x) = Macro.var(:y)
    end
  end

  def map([], acc, _) do
    {:ok, Enum.reverse(acc)}
  end

  def map([h | t], acc, func) when is_list(acc) do
    map(t, [func.(h) | acc])
  end

  def spawn_proc(x, y) do
    receive do
      {:message_type, ^x} ->
        :ok
      _ -> :fail
    end
    spawn_proc(x, y)
  end

  def main do
    map_v = %{
      :a => 2,
      :b => "Binary",
      :c => [2, 3, 4]
    }
    Enum.map(map_v, fn k -> k + 1 end)
    &(&1 + &2)
    case map_v do
      {:ok, ^f} -> map(map_v)
    end
    for i <- 1..100, do: spawn(__MODULE__, :spawn_proc, [2, :x])
  end

end
