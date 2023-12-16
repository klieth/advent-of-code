import Parser

namespace Data
open Parser

protected abbrev Parser := SimpleParser Substring Char

protected def all : Data.Parser (List (Float × Float)) :=
  withErrorMessage "expected all" do
    let _ <- Char.string "Time:" <* takeMany Char.ASCII.whitespace
    let times <- takeMany1 (Char.ASCII.parseNat <* takeMany Char.ASCII.whitespace)
    let _ <- Char.string "Distance:" <* takeMany Char.ASCII.whitespace
    let distances <- takeMany1 (Char.ASCII.parseNat <* takeMany Char.ASCII.whitespace)
    pure (List.zip (times.toList.map (·.toFloat)) (distances.toList.map (·.toFloat)))

protected def parse (raw : String) : Except String (List (Float × Float)) :=
  match Parser.run (Data.all <* Parser.endOfInput) raw.toSubstring with
  | .ok _ n => .ok n
  | .error e => .error s!"Failed! {e}"

end Data

def int_roots (a b c : Float) : (Float × Float) :=
  let discrim := ((b * b) - (4 * a * c)).sqrt
  let pos := ((b * -1) + discrim) / (2 * a)
  let neg := ((b * -1) - discrim) / (2 * a)
  -- exactly matching the existing record doesn't count, so we nudge them to account for that case.
  ((pos + 1).floor, (neg - 1).ceil)

def part1 (races : List (Float × Float)) : Float :=
  let roots := races.map (fun (t, r) => int_roots (-1) t (r * (-1)))
  let num_wins := roots.map (fun (l, h) => (h - l) + 1)
  num_wins.foldl (· * ·) 1

def int_concat (a b : Float) : Float :=
  (a * 10 ^ (Float.log10 b).ceil) + b

def part2 (races : List (Float × Float)) : Float :=
  let combined := races.foldl (fun (acct, accr) (t, r) => (int_concat acct t, int_concat accr r)) (0, 0)
  part1 [combined]

def main (args : List String) : IO Unit := do
  let filename <- match args.get? 0 with
    | .some f => pure f
    | .none => throw (IO.userError "no filename specified")
  let raw <- IO.FS.readFile filename
  let parsed <- IO.ofExcept (Data.parse raw)
  IO.println (part1 parsed)
  IO.println (part2 parsed)

#eval main ["../test"]
